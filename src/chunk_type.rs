use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    data: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.data
    }

//    0x20 是一个十六进制值，它对应于二进制的 00100000, PNG 规范中总是检查第五位
    pub fn is_critical(&self) -> bool {
        self.bytes()[0] & 0x20 != 0x20
    }

    pub fn is_public(&self) -> bool {
        self.bytes()[1] & 0x20 != 0x20
    }


    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes()[2] & 0x20 != 0x20
    }


    //  Safe-to-copy bit: bit 5 of fourth byte , 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes()[3] & 0x20 == 0x20
    }

    pub fn is_valid(&self) -> bool {
        let validated =  self.bytes().iter().all(|&byte|  Self::is_valid_byte(byte));
        validated && self.is_reserved_bit_valid()
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        byte >= b'a' && byte <= b'z' || (byte >= b'A' && byte <= b'Z')
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(bytes: [u8; 4]) -> Result<ChunkType> {
        for &byte in &bytes {
            if !ChunkType::is_valid_byte(byte) {
                return Err(Error::from("Invalid byte in chunk"));
            }
        }
        Ok(ChunkType {  data: bytes })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.data).unwrap_or("Invalid UTF-8"))
    }
}


impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        let data: [u8; 4] = bytes.try_into().map_err(|_| Error::from("ChunkType must be 4 bytes long"))?;
        Ok(ChunkType::try_from(data)?)
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