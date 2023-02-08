use std::fmt::{Debug, Formatter};
use std::io::Error;

//type Result<T> = std::result::Result<T, ChunkError>;
#[derive(Debug)]
pub enum ChunkError {
    BadCRC,
    TooShort,
    TooLong,
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
