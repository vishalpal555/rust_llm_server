use std::{fs::File, io};

use crate::utils;

pub fn read_tensor(file: &mut File, tensor_count: u64) -> io::Result<()> {
    for _ in 0..tensor_count {
        let name = utils::read_string(file)?;

        let n_dimensions = utils::read_u32(file)? as usize;

        let mut dimensions = Vec::with_capacity(n_dimensions);

        for _ in 0..n_dimensions {
            dimensions.push(utils::read_u64(file)?);
        }

        let tensor_type = utils::read_u32(file)?;
        let offset = utils::read_u64(file)?;

        println!("
        {{  name: {name},
            n_dimensions: {},
            dimensions: {:?},
            tensor_type: {},
            offset: {}
        }},", n_dimensions, dimensions, tensor_type, offset,);

        // store TensorInfo
    }
    Ok(())
}