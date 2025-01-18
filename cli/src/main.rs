use std::{fs::File, io::{Read, Write}, path::Path, str::FromStr};

use clap::{Args, Parser, Subcommand};
use core::{chunk::Chunk, chunk_type::ChunkType, png::Png};  

/// png tools
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Encode(EncodeArgs),
    Decode(CommandArgs),
    Remove(CommandArgs),
    Print(CommandArgs),
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



//  测试文件 /Users/kas/Downloads/IMG_0887.PNG

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Encode(args)) => {
            println!("encoding for {}",args.img_path);
            let  mut file = File::open(&args.img_path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();

            let mut png = Png::try_from(buffer.as_slice()).unwrap();
            let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();
            let chunk = Chunk::new(chunk_type, args.chunk_data.as_bytes().to_vec());
            png.append_chunk(chunk);

            let path = Path::new(&args.img_path);
            let path_buf = path.parent().map(|p| p.to_path_buf());
            if let Some( mut path_buf) = path_buf {
                path_buf.push(format!("{}.png",args.chunk_type ));
                println!("saved to {:?}", path_buf.display());
                let mut new_file = File::create(path_buf).unwrap();
                new_file.write_all(&png.as_bytes()).unwrap();
            } else {
                eprintln!("Failed to get parent directory.");
            }
            
        }

        Some(Commands::Decode(args)) => handle_list(args, 2, |list| {
            println!("Decoding the following lists: {:?}", list);
        }),

        Some(Commands::Remove(args)) => handle_list(args, 2, |list| {
            println!("Removing the following lists: {:?}", list);
        }),

        Some(Commands::Print(args)) => handle_list(args, 1, |list| {
            println!("Printing the following lists: {:?}", list);
        }),

        None => {
            eprintln!("No command was provided. Use --help for more information.");
        }
    }

    println!("done");
}


fn handle_list<F>(args: &CommandArgs,require: usize, logic: F)
where
    F: Fn(&Vec<String>),
{    
    if args.list.len() < require {
        eprintln!("\x1b[31mERROR\x1b[0m: Not enough arguments provided.");
    } else {
        logic(&args.list);
    }
}