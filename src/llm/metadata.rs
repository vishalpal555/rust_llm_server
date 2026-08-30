
use std::fs::File;
use std::io::{self, Read};

use crate::utils::{read_u32, read_u64};

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
    let mut last_element_processed: u64 = 0;
    for i in 0..metadata_count {
        let mut key_length_bytes: [u8; 8] = [0; 8];
        file.read_exact(&mut key_length_bytes)?;
        let key_length: usize = u64::from_le_bytes(key_length_bytes)
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, 
                "[read_meatadata]key_length exceeds maximum addressable memory"))?;
        
        let mut key_bytes: Vec<u8> = vec![0u8; key_length];
        file.read_exact(&mut key_bytes)?;
        let key = std::str::from_utf8(&key_bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        let mut value_type_bytes: [u8; 4] = [0; 4];
        file.read_exact(&mut value_type_bytes)?;
        let value_type: u32 = u32::from_le_bytes(value_type_bytes);
        
        let value = read_value(file, value_type)?;
        if value_type != 9 { // for now ignoring large array print
            println!("[read_meatadata]{key}: {value} ---> {value_type}");
        }
        last_element_processed = i + 1;
    }

    println!("[read_meatadata] completed {} elements", last_element_processed);
    Ok(())
}

fn read_value(file: &mut File, value_type: u32) -> io::Result<String> {
    match value_type {
        0 => {
            let mut u8_data: [u8; 1] = [0; 1];
            file.read_exact(&mut u8_data)?;
            Ok(u8_data[0].to_string())
        },
        1 => {
            let mut i8_data: [u8; 1] = [0; 1];
            file.read_exact(&mut i8_data)?;
            Ok((i8_data[0] as i8).to_string())
        },
        2 => {
            let mut u16_data: [u8; 2] = [0; 2];
            file.read_exact(&mut u16_data)?;
            Ok(u16::from_le_bytes(u16_data).to_string())
        },
        3 => {
            let mut i16_data: [u8; 2] = [0; 2];
            file.read_exact(&mut i16_data)?;
            Ok(i16::from_le_bytes(i16_data).to_string())
        },
        4 => {
            Ok(read_u32(file)?.to_string())
        },
        5 => {
            let mut i32_data: [u8; 4] = [0; 4];
            file.read_exact(&mut i32_data)?;
            Ok(i32::from_le_bytes(i32_data).to_string())
        },
        6 => {
            let mut f32_data: [u8; 4] = [0; 4];
            file.read_exact(&mut f32_data)?;
            Ok(f32::from_le_bytes(f32_data).to_string())
        },
        7 => {
            let mut bool_data: [u8; 1] = [0; 1];
            file.read_exact(&mut bool_data)?;
            Ok((bool_data[0] != 0).to_string())
        },
        8 => {
            // String: read length then data
            let mut len_bytes: [u8; 8] = [0; 8];
            file.read_exact(&mut len_bytes)?;
            let len = u64::from_le_bytes(len_bytes) as usize;
            let mut string_data = vec![0u8; len];
            file.read_exact(&mut string_data)?;
            Ok(String::from_utf8_lossy(&string_data).to_string())
        },
        9 => {
            let elem_type = read_u32(file)?;
            let array_size = read_u64(file)? as usize;
            
            let mut elements: Vec<String> = Vec::new();
            for _ in 0..array_size {
                let elem = read_value(file, elem_type)?;
                elements.push(elem);
            }
            
            Ok(format!("[{}]", elements.join(", ")))
        },
        10 => {
            Ok(read_u64(file)?.to_string())
        },
        11 => {
            let mut i64_data: [u8; 8] = [0; 8];
            file.read_exact(&mut i64_data)?;
            Ok(i64::from_le_bytes(i64_data).to_string())
        },
        12 => {
            let mut f64_data: [u8; 8] = [0; 8];
            file.read_exact(&mut f64_data)?;
            Ok(f64::from_le_bytes(f64_data).to_string())
        },
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, 
            format!("Unknown value type: {}", value_type)))
    }
}