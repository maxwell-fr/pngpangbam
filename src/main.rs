extern crate core;

mod chunk;
mod chunk_type;
mod png;
mod cli;
mod png_error;
mod chunk_error;

use clap::Parser;
use crate::cli::{Cli, CliSuccess};
use crate::png_error::PngError;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Cli::parse();


    match args.exec() {
        Ok(success) => {
            match success {
                CliSuccess::Success => {
                    println!("Done!");
                }
                CliSuccess::SuccessMsg(s) => {
                    println!("{s}");
                }
                CliSuccess::SuccessBytes(b) => {
                    println!("Bytes: {b:02X?}");
                }
                CliSuccess::SuccessHashMap(h) => {
                    println!("Chunks: {h:?}");
                }
            }
            Ok(())
        }
        Err(failure) => {
            match &failure {
                PngError::BadHeader => {
                    println!("Bad header.");
                }
                PngError::MissingRequiredChunks => {
                    println!("Malformed PNG: missing required chunks.");
                }
                PngError::ChunkNotFound => {
                    println!("Chunk not found.");
                }
                PngError::GenericError => {
                    println!("Unspecified error.");
                }
                PngError::IO(_) => {
                    println!("I/O error reading file.");
                }
                PngError::ChunkError(chunk) => {
                    println!("Chunk error: {chunk}");
                }
            }

            Ok(())
        }
    }

}
