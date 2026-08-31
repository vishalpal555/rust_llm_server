use std::{fs::File, io::{self, Seek}};

use crate::{constants, utils};

#[derive(Debug)]
pub struct TensorInfo {
    pub name: String,
    pub dimensions: Vec<u64>,
    pub tensor_type: u32,
    pub offset: u64,
}

pub fn read_tensor_info(file: &mut File, tensor_count: usize) -> io::Result<Vec<TensorInfo>> {
    let mut tensors = Vec::with_capacity(tensor_count);
    for _ in 0..tensor_count {
        let name = utils::read_string(file)?;

        let n_dimensions = utils::read_u32(file)? as usize;

        let mut dimensions = Vec::with_capacity(n_dimensions);

        for _ in 0..n_dimensions {
            dimensions.push(utils::read_u64(file)?);
        }

        let tensor_type = utils::read_u32(file)?;
        let offset = utils::read_u64(file)?;

        tensors.push(TensorInfo { 
            name, 
            dimensions, 
            tensor_type, 
            offset 
        });
    }
    Ok((tensors))
}

pub fn read_tensor_data(file: &mut File, general_alignment: u64) -> io::Result<()> {
    let current_pos = file.stream_position()?;

    let tensor_data_start = (current_pos + general_alignment - 1) /
        general_alignment * general_alignment;

    println!("Tensor data starts at: {}", tensor_data_start);
    Ok(())
}