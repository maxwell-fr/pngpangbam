
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod cli;
mod png_error;

use clap::Parser;
use crate::cli::Cli;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("Hello, world!");

    Ok(())
}
