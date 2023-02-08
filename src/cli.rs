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
        out_filename: Option<PathBuf>,
    },
    Print {
        filename: PathBuf,
    }
}

pub enum CliSuccess {
    Success,
    SuccessMsg(String),
    SuccessBytes(Vec<u8>),
}

impl From<()> for CliSuccess {
    fn from(_: ()) -> Self {
        CliSuccess::Success
    }
}



impl Cli {
    pub fn exec(&self) -> Result<CliSuccess, PngError> {
        match &self.command {
            PngCommand::Encode {filename, chunk_type, message, out_filename} => {
                let mut png = Png::load(filename)?;
                let ct = ChunkType::from_str(chunk_type)?;
                let new_chunk = Chunk::new(&ct, message.as_bytes().to_vec());
                let _ = png.remove_chunk(&ct); //try to remove the chunk if it exists; ignore error if it doesn't
                png.append_chunk(new_chunk);

                if let Some(out) = out_filename {
                    Ok(png.save(out)?.into())
                }
                else {
                    Ok(png.save(filename)?.into())
                }
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
                if let Some(out) = out_filename {
                    Ok(png.save(out)?.into())
                }
                else {
                    Ok(png.save(filename)?.into())
                }
            }
            PngCommand::Print { .. } => {
                Err(PngError::GenericError)
            }
        }



    }
}