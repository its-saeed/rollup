use anyhow::{Ok, Result};
use chainway::{create_merkle_tree, load_state, persist_state, update_state, Block, Cli};
use clap::Parser;
use colored::Colorize;
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut state = if cli.load_state {
        load_state()
    } else {
        [0_u64; 256]
    };

    let mut trusted_blocks = vec![];
    let mut on_da_pending_blocks = VecDeque::new();
    let mut on_da_approved_blocks = vec![];

    let mut da_file_reader = BufReader::new(std::fs::File::open(cli.da_file)?);
    let sequencer_file_reader = BufReader::new(std::fs::File::open(cli.sequencer_file)?);

    let mut sequencer_lied = 0_u8;
    let mut current_merkle_tree;
    for (index, line) in sequencer_file_reader.lines().enumerate() {
        println!("{}-{}", "seq".bold(), index + 1);
        let block = line?.parse::<Block>()?;
        // println!("{block:?}");
        update_state(&mut state, &block);
        current_merkle_tree = Some(create_merkle_tree(&state));
        // println!("{state:?}");
        trusted_blocks.push(block);

        if index % 5 == 4 {
            let index = (index / 5) + 1;
            println!("  {}-{}", "da".bold(), index);
            let mut line = String::new();
            let len = da_file_reader.read_line(&mut line)?;
            if len == 0 {
                continue;
            }
            if let Some(_) = &line.find("REORG") {
                let reorg_count: usize = line[6..7].parse()?;
                if on_da_pending_blocks.len() < reorg_count {
                    println!("  {}: Invalid reorg, only {} blocks are pending but reorg wants to invalidate {} blocks.",
                            "Error".bright_red(), on_da_pending_blocks.len(), reorg_count);
                } else {
                    on_da_pending_blocks.drain(on_da_pending_blocks.len() - reorg_count..);
                    println!("  {} {reorg_count}", "REORG".yellow());
                }

                // If reorg happened we need to read the next line again.
                line.clear();
                let len = da_file_reader.read_line(&mut line)?;
                if len == 0 {
                    continue;
                }
            }

            let block = line.trim_end().parse::<Block>()?;
            // println!("  {block:?}");
            update_state(&mut state, &block);
            // println!("  {state:?}");
            on_da_pending_blocks.push_back(block);
            if on_da_pending_blocks.len() == 4 {
                println!("  DA block {} approved", index - 3);
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
