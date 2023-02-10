//! This is an implementation of PNG chunk specific errors.

use std::fmt::{Debug, Formatter};

/// Implements specific errors emitted by the Chunk object.
#[derive(Debug)]
pub enum ChunkError {
    /// Chunk CRC does not match.
    BadCRC,
    /// Chunk length is shorter than the shortest possible chunk.
    TooShort,
    /// Chunk length value is longer than the actual data
    TooLong,
    /// Unspecified chunk error
    GenericError,
}

impl std::fmt::Display for ChunkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::BadCRC => write!(f, "Bad CRC."),
            ChunkError::TooShort => write!(f, "Chunk too short."),
            ChunkError::TooLong => write!(f, "Chunk too long."),
            ChunkError::GenericError => write!(f, "Non-specific chunk error."),
        }
    }
}

impl From<()> for ChunkError {
    fn from(_: ()) -> Self {
        ChunkError::GenericError
    }
}

impl std::error::Error for ChunkError {

}
