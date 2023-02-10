//! Module to handle normal commands, usually from a command-line interface.

use std::collections::HashMap;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::str::FromStr;
use crate::chunk::Chunk;
use crate::chunk::ChunkType;
use crate::png::{Png, PngError};

/// Contains the command logic and Clap parser handler.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: PngCommand,
}

/// Supported commands and their arguments
#[derive(Subcommand, Debug)]
pub enum PngCommand {
    /// Encode a message with the given chunk type.
    Encode {
        /// Path to source PNG file.
        filename: PathBuf,
        /// Chunk type code. See PNG spec at <http://www.libpng.org/pub/png/spec/1.2/> for details.
        chunk_type: String,
        /// Message to encode.
        message: String,
        /// Output file if different from source.
        out_filename: Option<PathBuf>,
    },
    /// Decode a message with the given chunk type.
    Decode {
        /// Path to source PNG file.
        filename: PathBuf,
        /// Chunk type code. See PNG spec at <http://www.libpng.org/pub/png/spec/1.2/> for details.
        chunk_type: String,
    },
    /// Remove the message with the given chunk type.
    Remove {
        /// Path to source PNG file.
        filename: PathBuf,
        /// Chunk type code. See PNG spec at <http://www.libpng.org/pub/png/spec/1.2/> for details.
        chunk_type: String,
        /// Output file if different from source.
        out_filename: Option<PathBuf>,
    },
    /// Generate a list of chunk types and their counts.
    Print {
        /// Path to source PNG file.
        filename: PathBuf,
    }
}

type SuccessHash = HashMap<String, u32>;

/// Possible successful outcomes.
pub enum CliSuccess {
    /// Simple all-good.
    Success,
    /// Success with a String result.
    SuccessMsg(String),
    /// Success with a byte vector result.
    SuccessBytes(Vec<u8>),
    /// Success with a key-value hashmap output.
    SuccessHashMap(SuccessHash),
}

impl From<()> for CliSuccess {
    fn from(_: ()) -> Self {
        CliSuccess::Success
    }
}



impl Cli {
    /// Initialize this object with Clap::Parser
    pub fn init() -> Cli {
        Cli::parse()
    }

    /// Execute the command contained in this object and return the outcome.
    pub fn exec(&self) -> Result<CliSuccess, PngError> {
        match &self.command {
            PngCommand::Encode {filename, chunk_type, message, out_filename} => {
                let mut png = Png::load(filename)?;
                let ct = ChunkType::from_str(chunk_type)?;
                let new_chunk = Chunk::new(&ct, message.as_bytes().to_vec());
                let _ = png.remove_chunk(&ct); //try to remove the chunk if it exists; ignore error if it doesn't
                png.append_chunk(new_chunk);

                let out_f = match out_filename {
                    None => filename,
                    Some(out) => out,
                };
                Ok(png.save(out_f)?.into())
            }
            PngCommand::Decode {filename, chunk_type } => {
                let png = Png::load(filename)?;
                let ct = ChunkType::from_str(chunk_type)?;

                match png.chunk_by_type(&ct) {
                    None => { Err(PngError::ChunkNotFound) }
                    Some(chunk) => {
                        if let Ok(cs) = chunk.as_string() {
                            Ok(CliSuccess::SuccessMsg(cs))
                        }
                        else {
                            Ok(CliSuccess::SuccessBytes(chunk.as_bytes()))
                        }
                    }
                }
            }
            PngCommand::Remove { filename, chunk_type, out_filename } => {
                let mut png = Png::load(filename)?;
                let ct = ChunkType::from_str(chunk_type)?;
                png.remove_chunk(&ct)?;

                let out_f = match out_filename {
                    None => filename,
                    Some(out) => out,
                };
                Ok(png.save(out_f)?.into())
            }
            PngCommand::Print { filename } => {
                let png = Png::load(filename)?;
                let mut hashmap = SuccessHash::new();
                for chunk in png.chunks() {
                    hashmap.entry(chunk.chunk_type().to_string()).and_modify(|ctr| *ctr += 1).or_insert(1);
                }

                Ok(CliSuccess::SuccessHashMap(hashmap))
            }
        }
    }
}