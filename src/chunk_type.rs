use std::str::FromStr;
use std::fmt;

use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    data: [u8; 4],
}

const MASK: u8 = 32;

impl ChunkType {
    
    pub fn bytes(&self) -> [u8; 4] {
        self.data
    }

    fn is_critical(&self) -> bool {
        self.data[0] & MASK == 0
    }

    fn is_public(&self) -> bool {
        self.data[1] & MASK == 0
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.data[2] & MASK == 0
    }

    fn is_safe_to_copy(&self) -> bool {
        self.data[3] & MASK != 0
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    fn is_alphanumerical(byte: u8) -> bool {
        (byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122)
    }

}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error<'static>;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        if bytes.iter().all(|&c| (c as char).is_ascii_alphabetic()) {
            Ok(ChunkType{data:bytes})
        } else {
            Err(Error::Other("Invalid assii literals"))
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error<'static>;

    fn from_str(input: &str) -> Result<Self> {
        if input.len() != 4 {
            return Err(Error::Other("Chunk type string must be exactly 4 characters long"));
        }

        // Convert the input string to bytes.
        let bytes = input.as_bytes();

        // Check if the resulting byte array has a length of 4 (4 bytes).
        if bytes.len() != 4 {
            return Err(Error::Other("Chunk type string must be exactly 4 characters long"));
        }
        // check all ar alphanumerical
        for byte in bytes {
            if !ChunkType::is_alphanumerical(*byte) {
                return Err(Error::Other("Invalid chunk type: must have only assi alphanumerical"));
            }
        }
        // Create a new ChunkType instance with the byte array.
        let mut chunk_as_slice = [0; 4];
        chunk_as_slice.clone_from_slice(bytes); 
        Ok(ChunkType { data: chunk_as_slice })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.data.to_vec().as_slice()));
        Ok(())
    }
}

#[cfg(test)]
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
        println!("{}", &chunk.to_string());
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
