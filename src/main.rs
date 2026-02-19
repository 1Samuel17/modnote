use anyhow::Result;
use clap::{Parser, Subcommand};
use modnote::db::set_db_options;
use sea_orm::Database;

/// A template for Rust CLI applications
#[derive(Parser, Debug)]
#[command(name = "modnote")]
#[command(version, about, long_about = None)]
struct Cli {
    /// Subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Create a new notebook or note
    New {
        #[arg(short = 'b', long, help = "Name of the notebook to create")]
        notebook: Option<String>,
        #[arg(short = 'n', long, help = "Name of the note to create")]
        note: Option<String>,
    },
    // Get an existing notebook or note
    Get {
        #[arg(short = 'b', long, help = "Name of the notebook to get")]
        notebook: Option<String>,
        #[arg(short = 'n', long, help = "Name of the note to get")]
        note: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // connect to the database
    let db_options = set_db_options();
    let _db = &Database::connect(db_options).await?;

    // init tracing
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::New { notebook, note }) => {
            if let Some(notebook) = notebook {
                println!("Created new notebook: {:?}", notebook);
            }
            if let Some(note_name) = note {
                println!("Created new note: {:?}", note_name);
            }
        }
        Some(Commands::Get { notebook, note }) => {
            if let Some(notebook) = notebook {
                println!("Getting notebook: {}", notebook);
            }
            if let Some(note_name) = note {
                println!("Getting note: {}", note_name);
            }
        }
        None => {
            println!("No subcommand was used. Use --help for more information.");
        }
    }

    Ok(())
}
