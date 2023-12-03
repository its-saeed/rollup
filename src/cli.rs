use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "DA_FILE")]
    pub da_file: PathBuf,

    #[arg(short, long, value_name = "SEQ_FILE")]
    pub sequencer_file: PathBuf,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub persist: bool,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub load_state: bool,
}
