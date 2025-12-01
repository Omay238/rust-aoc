use std::any::Any;
use clap::{Parser, Subcommand};

/// AOC project manager
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new AOC project in the specified directory.
    New {
        path: String
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        if command.type_id() == Commands::New {

        }
    }
}