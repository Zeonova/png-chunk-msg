use core::{chunk::Chunk, chunk_type::ChunkType, png::Png, Result};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

pub fn Encode(png_path: &str, chunk_type: &str, chunk_data: &str) -> Result<String> {
    let mut png = open_png(png_path)?;
    let c_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(c_type, chunk_data.as_bytes().to_vec());
    png.append_chunk(chunk);
    
    let path = Path::new(png_path);
    let mut path_buf = path
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or("There was an issue with path resolution.")?;
    path_buf.push(format!("{}.png",chunk_type ));
    let mut new_file = File::create(&path_buf)?;
    new_file.write_all(&png.as_bytes())?;
    Ok(path_buf.display().to_string())
}
pub fn Decode() {}

pub fn Remove() {}

pub fn Print() {}

fn open_png(path: &str) -> Result<Png> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Png::try_from(buffer.as_slice()).map_err(|e| e.into())
}
