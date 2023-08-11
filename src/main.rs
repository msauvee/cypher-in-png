mod chunk;
mod chunk_type;
mod commands;
mod crc;
mod error;
mod png;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "cypher-in-png")]
#[command(bin_name = "cypher-in-png")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // Decode a message from a png's chunk
    Decode {
        #[arg(required = true)]
        png_path: std::path::PathBuf,
        #[arg(required = true)]
        chunk: String,
    },
    // Encode a message into a png's chunk
    Encode {
        #[arg(required = true)]
        png_path: std::path::PathBuf,
        #[arg(required = true)]
        chunk: String,
        #[arg(required = true)]
        message: String,
    },
    // Remove a png's chunk
    Remove {
        #[arg(required = true)]
        png_path: std::path::PathBuf,
        #[arg(required = true)]
        chunk: String,
    },
    // Print a png's info
    Print {
        #[arg(required = true)]
        png_path: std::path::PathBuf
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Decode { png_path, chunk } => commands::decode(png_path.as_path(), chunk),
        Commands::Encode { png_path, chunk, message } => commands::encode(png_path.as_path(), chunk, message),
        Commands::Remove { png_path, chunk } => commands::remove(png_path.as_path(), chunk),
        Commands::Print { png_path } => commands::print(png_path.as_path()),
    };

}