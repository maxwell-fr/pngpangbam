use std::fmt::{Debug, Formatter};
use std::io::Error;
use crate::chunk_error::ChunkError;
use crate::png::Png;

//type Result<T> = std::result::Result<T, PngError>;
#[derive(Debug)]
pub enum PngError {
    BadHeader,
    MissingRequiredChunks,
    ChunkNotFound,
    GenericError,
    IO(std::io::Error),
    ChunkError(ChunkError),
}

impl std::fmt::Display for PngError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PngError::BadHeader => write!(f, "Bad, incomplete, or missing header."),
            PngError::MissingRequiredChunks => write!(f, "Missing required chunks."),
            PngError::ChunkNotFound => write!(f, "Chunk not found."),
            PngError::GenericError => write!(f, "Non-specific png error."),
            PngError::IO(e) => write!(f, "IO Error: {e}"),
            PngError::ChunkError(e) => {write!(f, "Chunk Error: {e}")},
        }
    }
}

impl From<std::io::Error> for PngError {
    fn from(value: Error) -> Self {
        PngError::IO(value)
    }
}

impl From<()> for PngError {
    fn from(_: ()) -> Self {
        PngError::GenericError
    }
}

impl From<ChunkError> for PngError {
    fn from(value: ChunkError) -> Self {
        PngError::ChunkError(value)
    }
}

impl std::error::Error for PngError {

}
