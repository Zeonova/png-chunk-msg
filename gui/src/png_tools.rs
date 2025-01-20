use core::{chunk::Chunk, chunk_type::ChunkType, png::Png, Result};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

pub fn encode(png_path: &str, chunk_type: &str, chunk_data: &str) -> Result<String> {
    let mut png = open_png(png_path)?;
    let c_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(c_type, chunk_data.as_bytes().to_vec());
    png.append_chunk(chunk);

    println!("{}", png.to_string());
    
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
pub fn decode(png_path: &str, chunk_type: &str) -> Result<String> {
    let png = open_png(png_path)?;
    print!("chunk_type : {}",chunk_type);
    let chunk = png.chunk_by_type(chunk_type).ok_or("Chunk Type not found.")?;
    let chunk_data_str = chunk.data_as_string()?;
    Ok(chunk_data_str)
}

pub fn remove(png_path: &str, chunk_type: &str) -> Result<String> {
    let mut png = open_png(png_path)?;
    png.remove_first_chunk(chunk_type)?;
    let mut new_file = File::create(png_path)?;
    new_file.write_all(&png.as_bytes())?;
    Ok(png_path.to_string())
}

pub fn print(png_path: &str)-> Result<String> {
    let png = open_png(png_path)?;
    Ok(png.to_string())
}

fn open_png(path: &str) -> Result<Png> {
    println!("Opening PNG file: {}", path);
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Png::try_from(buffer.as_slice()).map_err(|e| e.into())
}
