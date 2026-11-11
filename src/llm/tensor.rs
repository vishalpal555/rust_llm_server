use std::{fs::File, io::{self, Read, Seek}};

use crate::{utils};

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
    Ok(tensors)
}

pub fn read_tensor_data(
    file: &mut File,
    tensor_data_start: u64,
    tensor_info: &Vec<TensorInfo>
) -> io::Result<()> {
    for tensor in tensor_info {
        println!("{:?}", tensor);
        read_tensor(file, tensor, tensor_data_start)?;
        break;
    }
    Ok(())
}

fn read_tensor(
    file: &mut File,
    tensor: &TensorInfo,
    tensor_data_start: u64
) -> io::Result<()> {
    let position = tensor_data_start
        .checked_add(tensor.offset)
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::InvalidData,
            "Offset overflow",
        ))?;

    let size = tensor_nbytes(tensor)?;

    println!("tensor position: {position} and size: {size}");

    file.seek(std::io::SeekFrom::Start(position))?;
    
    let mut data = vec![0u8; size];

    file.read_exact(&mut data)?;
    
    println!("Tensor: {}", tensor.name);
    println!("Dimensions: {:?}", tensor.dimensions);
    println!("Type: {}", tensor.tensor_type);
    println!("Bytes: {}", size);
    
    Ok(())
}

pub fn tensor_nbytes(tensor: &TensorInfo) -> io::Result<usize> {
    const BLOCK_SIZE: u64 = 32;
    const BLOCK_BYTES: u64 = 17;

    let ne0 = *tensor.dimensions.first().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Empty tensor dimensions",
        )
    })?;

    if tensor.tensor_type != 39 {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Only MXFP4 is supported",
        ));
    }

    if ne0 % BLOCK_SIZE != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MXFP4 row dimension must be divisible by 32",
        ));
    }

    let rows = tensor.dimensions.iter()
        .skip(1)
        .try_fold(1u64, |acc, &dim| {
            acc.checked_mul(dim)
        })
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::InvalidData,
            "Dimension overflow",
        ))?;

    let bytes = (ne0 / BLOCK_SIZE)
        .checked_mul(BLOCK_BYTES)
        .and_then(|size| size.checked_mul(rows))
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::InvalidData,
            "Tensor size overflow",
        ))?;

    usize::try_from(bytes).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "Tensor too large")
    })
}