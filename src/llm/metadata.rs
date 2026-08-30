
use std::fs::File;
use std::io::{self, Read};

use crate::types::ValueType;
/*
 metadata entry
│
├── key length      u64
├── key bytes       [u8; key_length]
│
├── value type      u32
│
└── value           depends on value type
 */

 pub fn read_meatadata(file: &mut File, metadata_count: u64) -> io::Result<()> {
    for _ in 0..metadata_count {
        let mut key_length_bytes: [u8; 8] = [0; 8];
        file.read_exact(&mut key_length_bytes)?;
        let key_length: usize = u64::from_le_bytes(key_length_bytes)
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, 
                "key_length exceeds maximum addressable memory"))?;
        
        let mut key_bytes: Vec<u8> = vec![0u8; key_length];
        file.read_exact(&mut key_bytes)?;
        let key = std::str::from_utf8(&key_bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        let mut value_type_bytes: [u8; 4] = [0; 4];
        file.read_exact(&mut value_type_bytes)?;
        let value_type: u32 = u32::from_le_bytes(value_type_bytes);

        let value_type = ValueType::from_u32(value_type)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unknown value type: {}", value_type),
                )
            })?;
        
        println!("{key}: {:?}", value_type);
        break;
    }

    Ok(())
 }