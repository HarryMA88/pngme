use crate::chunk_type::ChunkType;
use crate::{Error, Result};
use std::fmt::{Debug, Display, Formatter};
use std::io::Read;

#[derive(Debug)]
pub struct InvalidByteSequence;

impl std::error::Error for InvalidByteSequence {}

impl Display for InvalidByteSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"InvalidByteSequence")
    }
}

pub struct Chunk {
    // based on https://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let byte_sequence: Vec<u8> = value.iter().copied().collect();
        let mut byte_sequence = &byte_sequence[..];

        // First 4 bytes = length of chunk data
        let mut length: [u8; 4] = [0; 4];
        byte_sequence.read(&mut length[..])?;
        let length = u32::from_be_bytes(length);

        // Next 4 bytes = chunk type
        let mut chunk_type_bytes: [u8; 4] = [0; 4];
        byte_sequence.read(&mut chunk_type_bytes[..])?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        // Based on length those next bytes will be for chunk data
        let mut chunk_data: Vec<u8> = vec![0; length as usize];
        byte_sequence.read(&mut chunk_data[..])?;

        // Last 4 bytes should be for the crc
        let mut crc: [u8; 4] = [0; 4];
        byte_sequence.read(&mut crc[..])?;
        let crc = u32::from_be_bytes(crc);

        let preceding_bytes: Vec<u8> = chunk_type_bytes.iter().chain(chunk_data.iter()).copied().collect();
        const X25: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let crc_check = X25.checksum(&preceding_bytes);

        if crc != crc_check {
            return Err(Box::new(InvalidByteSequence));
        }

        Ok((Chunk {
            length,
            chunk_type,
            chunk_data,
            crc,
        }))
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Length: {}, Type: {}, Data: {} bytes, Crc: {}",
            self.length,
            self.chunk_type,
            self.chunk_data.len(),
            self.crc,
        )
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len() as u32;

        let preceding_bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        const X25: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let crc = X25.checksum(&preceding_bytes);

        Chunk {
            length,
            chunk_type,
            chunk_data: data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.chunk_data.clone()).map_err(Box::new)?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let length: [u8; 4] = self.length.to_be_bytes();
        let chunk_type = self.chunk_type.bytes();
        let data = self.data();
        let crc: [u8; 4] = self.crc().to_be_bytes();

        let bytes: Vec<u8> = length
            .iter()
            .chain(chunk_type.iter())
            .chain(data.iter())
            .chain(crc.iter())
            .copied()
            .collect();

        bytes
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
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
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
