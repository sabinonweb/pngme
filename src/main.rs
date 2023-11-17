pub mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
pub mod utils;

use crate::args::Commands;
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // The parse method of Commands parses the CLI into the struct that implements the trait.
    // It does this by using std::env::args_os() iterator to iterate over and match them to args
    // in the struct
    let args = Commands::parse();
    commands::run(args.command);
    Ok(())
}
