use anyhow::{Ok, Result};
use chainway::{
    persistor::{load, persist},
    Block, Cli, Storage, SEQ_BLOCKS_PER_DA,
};
use clap::Parser;
use colored::Colorize;
use std::{
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

    let mut storage = if cli.load_state {
        load()?
    } else {
        Storage::new()
    };

    let mut da_file_reader = BufReader::new(std::fs::File::open(cli.da_file)?);
    let sequencer_file_reader = BufReader::new(std::fs::File::open(cli.sequencer_file)?);

    for (index, line) in sequencer_file_reader.lines().enumerate() {
        println!("{}-{}", "seq".bold(), index + 1);
        let block = line?.parse::<Block>()?;
        storage.add_trusted_block(block);

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
                    println!("  {} {reorg_count}", "REORG".yellow());
                    storage.reorg(reorg_count);

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
            storage.add_da_block(block);
        }
    }

    println!(
        "\n\n{}\n{}{}",
        storage,
        "DA Remaining: ".bold().bright_yellow(),
        da_file_reader.lines().count(),
    );

    if cli.persist {
        persist(&storage)
    } else {
        Ok(())
    }
}
