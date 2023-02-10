//! This is an implementation of PNG specific errors.

use std::fmt::{Debug, Formatter};

use crate::chunk::ChunkError;

/// Implements specific errors handled by the PNG object
#[derive(Debug)]
pub enum PngError {
    /// Bad or missing magic bytes.
    BadHeader,
    /// Non-optional chunks are missing (see <http://www.libpng.org/pub/png/spec/1.2/>).
    MissingRequiredChunks,
    /// Chunk sought by caller was not found.
    ChunkNotFound,
    /// Other unspecified error.
    GenericError,
    /// IO-specific error wrapper.
    IO(std::io::Error),
    /// Chunk-specific error wrapper.
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
    fn from(value: std::io::Error) -> Self {
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
