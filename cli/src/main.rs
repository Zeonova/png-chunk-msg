use std::{fs::File, io::{Read, Write}, path::Path, str::FromStr};

use clap::{Args, Parser, Subcommand};
use core::{chunk::Chunk, chunk_type::ChunkType, png::Png, Result};  

/// png tools
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode a chunk into a PNG file
    Encode(EncodeArgs),
    /// Decode a chunk from a PNG file
    Decode(DecodeOrRemoveArgs),
    /// Remove a chunk from a PNG file
    Remove(DecodeOrRemoveArgs),
    /// Print the binary of the PNG file
    Print{
        /// Path to the image file
        img_path: String,
    },
}

#[derive(Args)]
struct CommandArgs {
    list: Vec<String>,
}

#[derive(Args)]
struct EncodeArgs {
    /// Path to the image file
    img_path: String,
    /// Type of the chunk to add
    chunk_type: String,
    /// Data to add to the chunk 
    chunk_data: String,
}
#[derive(Args)]
struct DecodeOrRemoveArgs {
    /// Path to the image file
    img_path: String,
    /// Type of the chunk to operate on
    chunk_type: String,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Encode(args)) => {
            println!("encoding for {}",args.img_path);

            let mut png = open_png(&args.img_path).unwrap();
            let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();
            let chunk = Chunk::new(chunk_type, args.chunk_data.as_bytes().to_vec());
            png.append_chunk(chunk);

            let path = Path::new(&args.img_path);
            let path_buf = path.parent().map(|p| p.to_path_buf());
            if let Some( mut path_buf) = path_buf {
                path_buf.push(format!("{}.png",args.chunk_type ));
                let mut new_file = File::create(&path_buf).unwrap();
                new_file.write_all(&png.as_bytes()).unwrap();
                println!("Saved to {:?}", path_buf.display());
            } else {
                eprintln!("Failed to get parent directory.");
            }
            
        }

        Some(Commands::Decode(args)) => {
            let png = open_png(&args.img_path).unwrap();
            let chunk = png.chunk_by_type(&args.chunk_type).unwrap();

            if let Ok(chunk_data_str) = chunk.data_as_string() {
                println!("Decoded chunk to String: {}", chunk_data_str);
            } else {
                eprintln!("Decoded failed.");
            }

        },

        Some(Commands::Remove(args)) => {

            let mut png = open_png(&args.img_path).unwrap();
            if let Ok(_) = png.remove_first_chunk(&args.chunk_type) {
                let mut new_file = File::create(&args.img_path).unwrap();
                new_file.write_all(&png.as_bytes()).unwrap();
                println!("Removed chunk: {} and overwrote {}", args.chunk_type , args.img_path);
            } else {
                eprintln!("Failed to remove chunk.");
            }
        },

        // 注意这里和上面的区别，Print命令是结构体，而不是枚举
        Some(Commands::Print { img_path }) =>{
            let png = open_png(img_path).unwrap();
            println!("{}", png);
        },

        None => {
            eprintln!("No command was provided. Use --help for more information.");
        }
    }

    println!("done");
}


fn open_png(path: &str) -> Result<Png> {
    let  mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Png::try_from(buffer.as_slice()).map_err(|e| e.into())
}


#[cfg(test)]
mod tests {
    pub const CHUNK_TEST_DATA: &str = "Hello, world!";


#[test]
fn test_encode_png() {
    let img_path = "382px-PNG_Test.png";
    let chunk_type = "ruSt";


    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("encode")
        .arg(img_path)
        .arg(chunk_type)
        .arg(CHUNK_TEST_DATA)
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Saved"));
}

#[test]
fn test_decode_png() {
    let img_path = "ruSt.png";
    let chunk_type = "ruSt";
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("decode")
        .arg(img_path)
        .arg(chunk_type)
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(CHUNK_TEST_DATA));
}
}