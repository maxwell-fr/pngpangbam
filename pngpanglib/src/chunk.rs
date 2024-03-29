//! Describes and implements a PNG chunk.
//!
//! The chunk is the building block of a PNG data structure and has certain qualities
//! that make it valid or invalid.

mod chunk_type;
mod chunk_error;

use std::fmt::{Display, Formatter};
use std::io::Write;
use std::string::FromUtf8Error;
use crc::Crc;

pub use chunk_error::ChunkError;
pub use chunk_type::ChunkType;

type Result<T> = std::result::Result<T, ChunkError>;

/// Each chunk is a structured component of a PNG file.
pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32
}

impl Chunk {
    /// Create a new chunk with the provided data.
    pub fn new(chunk_type: &ChunkType, data: Vec<u8>) -> Chunk {
        let mut no_crc_chunk = Chunk {
            data_length: data.len() as u32,
            chunk_type: chunk_type.clone(),
            data,
            crc: 0,
        };

        no_crc_chunk.crc = no_crc_chunk.crc();
        no_crc_chunk
    }

    /// Get the length of the data in the chunk.
    pub fn length(&self) -> u32 {
        self.data_length
    }

    /// Get the chunk type.
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    // pub fn data(&self) -> &[u8] {
    //     let r: &[u8] = &self.data;
    //     r
    // }

    /// Compute CRC of data and header.
    fn crc(&self) -> u32 {
        let chk = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut dh: Vec<u8> = self.chunk_type.bytes().to_vec();
        dh.append(&mut self.data.clone());
        chk.checksum(dh.by_ref())
    }

    /// Return the data as String, if possible.
    pub fn as_string(&self) -> std::result::Result<String, FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    /// Return a copy of the data as a byte vector.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut byt: Vec<u8> = self.data_length.to_be_bytes().to_vec();
        byt.append(&mut self.chunk_type.bytes().to_vec());
        byt.append(&mut self.data.clone());
        byt.append(&mut self.crc.to_be_bytes().to_vec());

        byt
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(value: &[u8]) -> Result<Self> {
        //length, type, data, crc
        if value.len() < 12 { //len, type, crc. data can be 0
            return Err(ChunkError::TooShort);
        }
        let data_length: u32 = u32::from_be_bytes((&value[0..4]).try_into().unwrap());

        if data_length > value.len() as u32 - 12 {
            return Err(ChunkError::TooLong);
        }

        type Offsets = (usize, usize);
        let type_offsets: Offsets = (4, 8);
        let data_offsets: Offsets = (8, data_length as usize + 8);
        let crc_offsets: Offsets = (data_offsets.1, data_offsets.1 + 4);

        let chunk_type = ChunkType::try_from(<[u8; 4]>::try_from(&value[type_offsets.0 .. type_offsets.1]).unwrap()).unwrap();
        let data = Vec::from(&value[data_offsets.0 .. data_offsets.1]);
        let crc = u32::from_be_bytes((&value[crc_offsets.0 .. crc_offsets.1]).try_into().unwrap());

        let chunk = Chunk {
            data_length,
            chunk_type,
            data,
            crc,
        };

        if chunk.crc() != crc {
            return Err(ChunkError::BadCRC);
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "len: {}  type: {}  crc: {}  data: {}",
               self.data_length, self.chunk_type, self.crc, self.as_string().expect("<non-msg data>"))
    }
}

/// These tests use those provided by picklenerd <https://picklenerd.github.io/pngme_book/> as a foundation.
#[cfg(test)]
mod tests {
    use super::*;

    fn testing_bytes(good: bool) -> Vec<u8> {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = if good {
            2882656334
        }
        else {
            2882656333
        };

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        chunk_data
    }

    fn testing_chunk() -> Chunk {
        Chunk::try_from(testing_bytes(true).as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let chunk = Chunk::try_from(testing_bytes(true).as_ref()).unwrap();

        let chunk_string = chunk.as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let chunk = Chunk::try_from(testing_bytes(false).as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let chunk: Chunk = Chunk::try_from(testing_bytes(true).as_ref()).unwrap();

        let _chunk_string = format!("{chunk}");
    }
}


