use anyhow::Result;
use clap::{Parser, Subcommand};
use modnote::notebook::{Notebooks, Notebook, Note};

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
    New {
        #[arg(short = 'b', long, help = "Name of the notebook to create")]
        notebook: Option<String>,
        #[arg(short = 'n', long, help = "Name of the note to create")]
        note: Option<String>,
    },
    Get {
        #[arg(short = 'b', long, help = "Name of the notebook to get")]
        notebook: Option<String>,
        #[arg(short = 'n', long, help = "Name of the note to get")]
        note: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let notebooks = Notebooks::new("Master Collection".to_string());
    let default_notebook = Notebook::default();

    if cli.verbose {
        println!("Running in verbose mode...");
    }

    match &cli.command {
        Some(Commands::New { notebook, note }) => {
            if let Some(notebook_name) = notebook {
                println!("Creating new notebook: {}", notebook_name);
                // Here you would add logic to create and save the notebook
            }
            if let Some(note_name) = note {
                println!("Creating new note: {}", note_name);
                // Here you would add logic to create and save the note
            }
        }
        Some(Commands::Get { notebook, note }) => {
            if let Some(notebook_name) = notebook {
                println!("Getting notebook: {}", notebook_name);
                // Here you would add logic to get the notebook
            }
            if let Some(note_name) = note {
                println!("Getting note: {}", note_name);
                // Here you would add logic to get the note
            }
        }
        None => {
            println!("No subcommand was used. Use --help for more information.");
        }
    }

    Ok(())
}
