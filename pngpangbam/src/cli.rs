use clap::Parser;

use pngpanglib::png_command::{PngCommand, PngCmdSuccess};
use pngpanglib::png::PngError;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: PngCommand,
}

impl Cli {
    pub fn init() -> Cli {
        Cli::parse()
    }

    pub fn exec(&self) -> Result<PngCmdSuccess, PngError> {
        PngCommand::exec(&self.command)
    }

    pub fn exec_and_display(&self) {
        match self.exec() {
            Ok(success) => {
                match success {
                    PngCmdSuccess::Success => {
                        println!("Done!");
                    }
                    PngCmdSuccess::SuccessMsg(s) => {
                        println!("{s}");
                    }
                    PngCmdSuccess::SuccessBytes(b) => {
                        println!("Bytes: {b:02X?}");
                    }
                    PngCmdSuccess::SuccessHashMap(h) => {
                        println!("Chunks: {h:?}");
                    }
                }
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
            }
        }
    }

    pub fn run() {
        let cmd = Cli::init();

        cmd.exec_and_display();
    }
}