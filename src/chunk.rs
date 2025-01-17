use std::fmt;
use crate::{chunk_type::ChunkType, Error, Result};

#[derive(Debug, PartialEq,)]
pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    pub const DATA_LENGTH_SIZE: usize = 4;
    pub const CHUNK_TYPE_SIZE: usize = 4;
    // data  size  >= 0 
    pub const CRC_SIZE: usize = 4;
    pub const METADATA_SIZE : usize = Chunk::DATA_LENGTH_SIZE + Chunk::CHUNK_TYPE_SIZE + Chunk::CRC_SIZE;
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk {
            chunk_type,
            data,
        }
    }

    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType{
        &self.chunk_type
    }

    fn data(&self) -> &[u8]{ 
        &self.data 
    }
// 规范中只计算 chunk type 和 data 部分的 CRC，不包括 length 和 CRC 字段
    fn crc(&self) -> u32{
        let mut digest = crc32fast::Hasher::new();
        digest.update(&self.chunk_type.bytes());
        digest.update(&self.data);
        digest.finalize()
    }
    pub fn data_as_string(&self) -> Result<String> {
        String::from_utf8(self.data.clone()).map_err(|_| Error::from("Invalid UTF-8"))
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.length().to_be_bytes());
        bytes.extend_from_slice(&self.chunk_type.bytes());
        bytes.extend_from_slice(&self.data);
        bytes.extend_from_slice(&self.crc().to_be_bytes());
        bytes
    }

}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < Chunk::METADATA_SIZE {
            return Err(Error::from("Chunk data is too short"));
        }
        
        let (data_length, rest) = bytes.split_at(Chunk::DATA_LENGTH_SIZE);
        let data_length = u32::from_be_bytes(data_length.try_into()?);


        let (chunk_type, rest) = rest.split_at(Chunk::CHUNK_TYPE_SIZE);

        let chunk_type_bytes: [u8; 4] = chunk_type.try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        if !chunk_type.is_valid() {
            return Err(Error::from("Invalid chunk type"));
        }
        let (data, rest) = rest.split_at(data_length as usize);
        let (crc_bytes, _) = rest.split_at(Chunk::CRC_SIZE);

        let new = Self {
            chunk_type,
            data: data.into(),
        };

        let actual_crc = new.crc();
        let expected_crc = u32::from_be_bytes(crc_bytes.try_into()?);

        if actual_crc != expected_crc {
            return Err(Error::from("Invalid CRC"));
        }
        Ok(new)
    }
    
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.chunk_type, self.data_as_string().unwrap_or_else(|_| "Invalid UTF-8".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;
    
    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        println!("{:?}", chunk);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
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
        let chunk_string = chunk.data_as_string().unwrap();
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
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}