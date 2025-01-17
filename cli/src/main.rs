use clap::{Args, Parser, Subcommand};

/// png tools
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Encode(CommandArgs),
    Decode(CommandArgs),
    Remove(CommandArgs),
    Print(CommandArgs),
}

#[derive(Args)]
struct CommandArgs {
    list: Vec<String>,
}
//TODO: 注意检查一下追加自定块是，放在IEND块前后的区别，clap是否支持自动推断参数？
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Encode(args)) => handle_list(args, 3, |list| {
            println!("Encoding the following lists: {:?}", list);
        }),

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