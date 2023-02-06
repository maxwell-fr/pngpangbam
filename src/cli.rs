use clap::{Parser,Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: PngCommand,
}

#[derive(Subcommand, Debug)]
pub enum PngCommand {
    Encode {
        filename: PathBuf,
        chunk_type: String,
        message: String,
        out_filename: Option<PathBuf>,
    },
    Decode {
        filename: PathBuf,
        chunk_type: String,
    },
    Remove {
        filename: PathBuf,
        chunk_type: String,
    },
    Print {
        filename: PathBuf,
    }
}