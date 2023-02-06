use clap::{Parser,Subcommand};
use std::path::PathBuf;
use std::str::FromStr;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::png_error::PngError;

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

impl Cli {
    pub fn exec(&self) -> Result<(), PngError> {
        match &self.command {
            PngCommand::Encode {filename, chunk_type, message, out_filename} => {
                let mut png = Png::load(filename)?;
                let ct = ChunkType::from_str(chunk_type)?;
                let new_chunk = Chunk::new(&ct, message.as_bytes().to_vec());
                let _ = png.remove_chunk(&ct.to_string()); //try to remove the chunk if it exists; ignore error if it doesn't
                png.append_chunk(new_chunk);

                if let Some(out) = out_filename {
                    png.save(out)?;
                }
                else {
                    png.save(filename)?;
                }
            }
            PngCommand::Decode {filename, chunk_type } => {
                let png = Png::load(filename)?;
                let ct = ChunkType::from_str(chunk_type)?;

                match png.chunk_by_type(&ct.to_string()) {
                    None => { println!("Not found!") }
                    Some(chunk) => { println!("Found {}: {}", chunk.chunk_type(), chunk.data_as_string().unwrap_or("could not convert to String".to_string()))}
                }
            }
            PngCommand::Remove { .. } => {}
            PngCommand::Print { .. } => {}
        }

        Ok(())
    }
}