use anyhow::Result;
use clap::{Parser, Subcommand};

/// A template for Rust CLI applications
#[derive(Parser, Debug)]
#[command(name = "modnote")]
#[command(version, about, long_about = None)]
struct Cli {
    /// Subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Greet {
        /// Name to greet
        #[arg(short, long, default_value = "World")]
        name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Running in verbose mode...");
    }

    match &cli.command {
        Some(Commands::Greet { name }) => {
            println!("Hello, {}!", name);
        }
        None => {
            println!("No subcommand was used. Use --help for more information.");
        }
    }

    Ok(())
}
