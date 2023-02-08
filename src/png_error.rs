use std::fmt::{Debug, Formatter};
use std::io::Error;

//type Result<T> = std::result::Result<T, PngError>;
#[derive(Debug)]
pub enum PngError {
    BadHeader,
    MissingRequiredChunks,
    ChunkNotFound,
    GenericError,
    IO(std::io::Error),
}

impl std::fmt::Display for PngError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PngError::BadHeader => write!(f, "Bad, incomplete, or missing header."),
            PngError::MissingRequiredChunks => write!(f, "Missing required chunks."),
            PngError::ChunkNotFound => write!(f, "Chunk not found."),
            PngError::GenericError => write!(f, "Non-specific generic error."),
            PngError::IO(e) => write!(f, "IO Error: {e}"),
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

impl std::error::Error for PngError {

}
