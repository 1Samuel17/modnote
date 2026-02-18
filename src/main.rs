use anyhow::Result;
use clap::{Parser, Subcommand};
use modnote::{
    db::set_db_options,
    notebook::{Note, Notebook, Collection},
};
use sea_orm::Database;

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

    // create database tables from entities
    // db.get_schema_registry("modnote::entity::*").sync(db).await?;

    // init tracing
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();

    let cli = Cli::parse();
    // Initialize the master collection of notebooks
    let mut master_collection = Collection::new("Master Collection".to_string(), "Master Default Collection".to_string());
    // Initialize a default notebook as a catch-all for unorganized notes
    let mut default_notebook = Notebook::default();

    if cli.verbose {
        println!("Running in verbose mode...");
    }

    match &cli.command {
        Some(Commands::New { notebook, note }) => {
            if let Some(notebook) = notebook {
                // Create a new notebook and add it to the master collection
                let notebook = Notebook::new(notebook.clone());
                master_collection.add_notebook(notebook.clone());
                println!("Created new notebook: {:?}", notebook);
            }
            if let Some(note_name) = note {
                // Create a new note and add it to the default notebook
                let note_name = Note::new(note_name.clone(), "Description".to_string());
                default_notebook.add_note(note_name.clone());
                println!("Created new note: {:?}", note_name);
            }
        }
        Some(Commands::Get { notebook, note }) => {
            if let Some(notebook) = notebook {
                // Get the notebook from the master collection
                master_collection.get_notebook(notebook);
                println!("Getting notebook: {}", notebook);
            }
            if let Some(note_name) = note {
                // Get the note from the default notebook
                default_notebook.get_note(note_name);
                println!("Getting note: {}", note_name);
            }
        }
        None => {
            println!("No subcommand was used. Use --help for more information.");
        }
    }

    Ok(())
}
