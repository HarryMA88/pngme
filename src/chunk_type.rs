use crate::{Error, Result};
use std::fmt::{Display, Formatter};
use std::str;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    // based on https://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
    type_code: [u8; 4],
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.type_code).unwrap())
    }
}

impl FromStr for ChunkType {
    type Err = InvalidTypeCodeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(InvalidTypeCodeError);
        }

        let mut type_code: [u8; 4] = [0; 4];
        for (i, c) in s.chars().enumerate() {
            if c.is_ascii_uppercase() || c.is_ascii_lowercase() {
                type_code[i] = c as u8;
            } else {
                return Err(InvalidTypeCodeError);
            }
        }

        Ok(ChunkType { type_code })
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(type_code: [u8; 4]) -> Result<ChunkType> {
        for byte in type_code {
            if !byte.is_ascii_lowercase() && !byte.is_ascii_uppercase() {
                return Err(Box::new(InvalidTypeCodeError));
            }
        }

        Ok(ChunkType { type_code })
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.type_code
    }

    pub fn is_valid(&self) -> bool {
        for byte in self.type_code {
            if !byte.is_ascii_lowercase() && !byte.is_ascii_uppercase() {
                return false;
            }
        }

        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        let ancillary_byte = self.type_code[0];
        if ancillary_byte & (1 << 5) != 0 {
            return false;
        }
        true
    }

    pub fn is_public(&self) -> bool {
        let public_byte = self.type_code[1];
        if public_byte & (1 << 5) != 0 {
            return false;
        }

        true
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let reserved_byte = self.type_code[2];
        if reserved_byte & (1 << 5) != 0 {
            return false;
        }

        true
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let safe_to_copy_byte = self.type_code[3];
        if safe_to_copy_byte & (1 << 5) == 0 {
            return false;
        }

        true
    }
}

#[derive(Debug)]
pub struct InvalidTypeCodeError;

impl std::error::Error for InvalidTypeCodeError {}

impl Display for InvalidTypeCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InvalidTypeCodeError: please use uppercase and lowercase ASCII letters only",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_is_equal() {
        let chunk1 = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let chunk2 = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(chunk1, chunk2);
    }

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
