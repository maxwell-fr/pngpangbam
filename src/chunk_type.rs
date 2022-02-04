pub mod chunk_type {

    #[derive(Debug)]
    pub struct ChunkType {
        type_code: [u8; 4],
    }

    use std::convert::TryFrom;
    use std::fmt::{Display, Formatter};
    use std::str::FromStr;


    impl ChunkType {
        pub fn bytes(&self) -> [u8; 4] {
            self.type_code
        }

        pub fn is_valid(&self) -> bool {
            self.type_code[2] & 0b100000 == 0
        }

        fn is_critical(&self) -> bool {
            self.type_code[0] & 0b100000 == 0
        }

        fn is_public(&self) -> bool {
            self.type_code[1] & 0b100000 == 0
        }

        fn is_reserved_bit_valid(&self) -> bool {
            self.type_code[2] & 0b100000 == 0
        }

        fn is_safe_to_copy(&self) -> bool {
            self.type_code[3] & 0b100000 != 0
        }
    }

    impl TryFrom<[u8; 4]> for ChunkType {
        type Error = ();
        fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
            Ok(ChunkType { type_code: value })
        }
    }

    impl FromStr for ChunkType {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let bytes = s.as_bytes();
            if bytes.len() != 4 {
                Err(())
            }
            else {
                for &b in bytes {
                    if !((b >= 65 && b <= 90) || (b >= 97 && b <= 122)) {
                        return Err(());
                    }
                }
                Ok(ChunkType { type_code: [bytes[0], bytes[1], bytes[2], bytes[3]] })
            }
        }
    }

    impl Display for ChunkType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let s = self.type_code;
            write!(f, "{}{}{}{}", s[0] as char, s[1] as char, s[2] as char, s[3] as char)
        }
    }

    impl PartialEq for ChunkType {
        fn eq(&self, other: &Self) -> bool {
            self.type_code[0] == other.type_code[0] &&
                self.type_code[1] == other.type_code[1] &&
                self.type_code[2] == other.type_code[2] &&
                self.type_code[3] == other.type_code[3]
        }

        fn ne(&self, other: &Self) -> bool {
            self.type_code[0] != other.type_code[0] ||
                self.type_code[1] != other.type_code[1] ||
                self.type_code[2] != other.type_code[2] ||
                self.type_code[3] != other.type_code[3]
        }
    }

    impl Eq for ChunkType {}


    //picklenerd's tests below
    mod tests {
        use super::*;
        use std::convert::TryFrom;
        use std::str::FromStr;

        #[test]
        pub fn test_chunk_type_from_bytes() {
            let expected = [82, 117, 83, 116];
            let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

            assert_eq!(expected, actual.bytes());
        }

        #[test]
        pub fn test_chunk_type_from_str() {
            let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
            let actual = ChunkType::from_str("RuSt").unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        pub fn test_chunk_type_is_critical() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert!(chunk.is_critical());
        }

        #[test]
        pub fn test_chunk_type_is_not_critical() {
            let chunk = ChunkType::from_str("ruSt").unwrap();
            assert!(!chunk.is_critical());
        }

        #[test]
        pub fn test_chunk_type_is_public() {
            let chunk = ChunkType::from_str("RUSt").unwrap();
            assert!(chunk.is_public());
        }

        #[test]
        pub fn test_chunk_type_is_not_public() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert!(!chunk.is_public());
        }

        #[test]
        pub fn test_chunk_type_is_reserved_bit_valid() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert!(chunk.is_reserved_bit_valid());
        }

        #[test]
        pub fn test_chunk_type_is_reserved_bit_invalid() {
            let chunk = ChunkType::from_str("Rust").unwrap();
            assert!(!chunk.is_reserved_bit_valid());
        }

        #[test]
        pub fn test_chunk_type_is_safe_to_copy() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert!(chunk.is_safe_to_copy());
        }

        #[test]
        pub fn test_chunk_type_is_unsafe_to_copy() {
            let chunk = ChunkType::from_str("RuST").unwrap();
            assert!(!chunk.is_safe_to_copy());
        }

        #[test]
        pub fn test_valid_chunk_is_valid() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert!(chunk.is_valid());
        }

        #[test]
        pub fn test_invalid_chunk_is_valid() {
            let chunk = ChunkType::from_str("Rust").unwrap();
            assert!(!chunk.is_valid());

            let chunk = ChunkType::from_str("Ru1t");
            assert!(chunk.is_err());

        }

        #[test]
        pub fn test_chunk_type_string() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert_eq!(&chunk.to_string(), "RuSt");
        }

        #[test]
        pub fn test_chunk_type_trait_impls() {
            let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
            let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
            let _chunk_string = format!("{}", chunk_type_1);
            let _are_chunks_equal = chunk_type_1 == chunk_type_2;
        }
    }
}