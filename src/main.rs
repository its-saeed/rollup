use anyhow::{Ok, Result};
use chainway::{create_merkle_tree, load_state, persist_state, update_state, Block, Cli};
use clap::Parser;
use colored::Colorize;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn da_next_line(da_file_reader: &mut BufReader<File>) -> Result<String> {
    let mut line = String::new();
    da_file_reader.read_line(&mut line)?;
    Ok(line)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    const SEQ_BLOCKS_PER_DA: usize = 5;

    let mut state = if cli.load_state {
        load_state()?
    } else {
        [0_u64; 256]
    };

    let mut trusted_blocks = VecDeque::new();
    let mut on_da_pending_blocks = VecDeque::new();
    let mut on_da_approved_blocks = vec![];

    let mut da_file_reader = BufReader::new(std::fs::File::open(cli.da_file)?);
    let sequencer_file_reader = BufReader::new(std::fs::File::open(cli.sequencer_file)?);

    let mut sequencer_lied = 0_u8;
    let mut current_merkle_tree;
    for (index, line) in sequencer_file_reader.lines().enumerate() {
        println!("{}-{}", "seq".bold(), index + 1);
        let block = line?.parse::<Block>()?;
        update_state(&mut state, &block);
        current_merkle_tree = Some(create_merkle_tree(&state));
        trusted_blocks.push_back(block);

        if index % SEQ_BLOCKS_PER_DA == 4 {
            let index = (index / SEQ_BLOCKS_PER_DA) + 1;
            println!("  {}-{}", "da".bold(), index);
            let line = da_next_line(&mut da_file_reader)?;
            if line.is_empty() {
                continue;
            }
            let next_line_to_parse = match &line.find("REORG") {
                Some(_) => {
                    let reorg_count: usize = line[6..7].parse()?;
                    if on_da_pending_blocks.len() < reorg_count {
                        println!("  {}: Invalid reorg, only {} blocks are pending but reorg wants to invalidate {} blocks.",
                            "Error".bright_red(), on_da_pending_blocks.len(), reorg_count);
                    } else {
                        on_da_pending_blocks.drain(on_da_pending_blocks.len() - reorg_count..);
                        trusted_blocks.drain(
                            trusted_blocks.len() - ((reorg_count + 1) * SEQ_BLOCKS_PER_DA)
                                ..trusted_blocks.len() - SEQ_BLOCKS_PER_DA,
                        );
                        println!("  {} {reorg_count}", "REORG".yellow());
                    }

                    // If reorg happened we need to read the next line again.
                    let line = da_next_line(&mut da_file_reader)?;
                    if line.is_empty() {
                        continue;
                    }

                    line
                }
                None => line,
            };

            let block = next_line_to_parse.trim_end().parse::<Block>()?;
            update_state(&mut state, &block);
            on_da_pending_blocks.push_back(block);
            if on_da_pending_blocks.len() == 4 {
                trusted_blocks.drain(..SEQ_BLOCKS_PER_DA);
                println!("  DA block {} approved {}", index - 3, trusted_blocks.len());
                on_da_approved_blocks.push(on_da_pending_blocks.pop_front());
            }

            if let Some(tree) = current_merkle_tree {
                let next_tree = create_merkle_tree(&state);
                if tree.root_hex() != next_tree.root_hex() {
                    println!("{}", "  Sequencer lied".red());
                    sequencer_lied += 1;
                }
                // current_merkle_tree = Some(next_tree);
            }
        }
    }

    if cli.persist {
        persist_state(&state)?;
    }

    println!(
        "\n\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}",
        "Trusted Blocks: ".bold(),
        trusted_blocks.len(),
        "Pending blocks: ".bold().yellow(),
        on_da_pending_blocks.len(),
        "Approved blocks: ".bold().bright_green(),
        on_da_approved_blocks.len(),
        "DA Remaining: ".bold().bright_yellow(),
        da_file_reader.lines().count(),
        "Sequencer lied: ".bold().red(),
        sequencer_lied
    );

    Ok(())
}
