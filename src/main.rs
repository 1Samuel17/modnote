use anyhow::Result;
use clap::{Parser, Subcommand};
use modnote::{crud::notebook::create_notebook, db::set_db_options};
use sea_orm::Database;
use tracing_subscriber;
use std::env;

// A template for Rust CLI applications
#[derive(Parser, Debug)]
#[command(name = "modnote")]
#[command(version, about, long_about = None)]
struct Cli {
    // Subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,

    // Create
    #[arg(short, long, help = "Set flag to create a new notebook or note")]
    new: bool,

    // Read
    #[arg(short, long, help = "Set flag to get an existing notebook or note")]
    get: bool,

    // Update
    #[arg(short, long, help = "Set flag to update an existing notebook or note")]
    up: bool,

    // Delete
    #[arg(short, long, help = "Set flag to delete an existing notebook or note")]
    del: bool,

}

#[derive(Subcommand, Debug)]
enum Commands {
    // Notebook subcommand for creating, getting, updating, or deleting notebooks
    Notebook {
        #[arg(short = 't', long = "title", help = "Provide the title of the notebook")]
        title: String,

        #[arg(short = 'd', long, help = "Provide a short description of the notebook")]
        description: Option<String>,
    },
    // Note subcommand for creating, getting, updating, or deleting notes
    Note {
        #[arg(short = 't', long = "title", help = "Provide the title of the note to get")]
        title: String,

        #[arg(short = 'c', long = "content", help = "Add note content")]
        content: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // connect to the database
    let db_options = set_db_options();
    let db = &Database::connect(db_options).await?;

    // initialize tracing
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Notebook {title, description }) => {
            let args = env::args();
            match args {
                arg if cli.new => {
                    create_notebook(db, title, description).await?;
                    println!("successfully added new notebook");
                }
                arg if cli.get => {
                    println!("get flag checked")
                }
                arg if cli.up => {
                    println!("up flag checked")
                }
                arg if cli.del => {
                    println!("del flag checked")
                }
                _ => {()}
            }
        }
        Some(Commands::Note {title, content }) => {
            let args = env::args();
            match args {
                arg if cli.new => {
                    println!("new flag checked");
                }
                arg if cli.get => {
                    println!("get flag checked")
                }
                arg if cli.up => {
                    println!("up flag checked")
                }
                arg if cli.del => {
                    println!("del flag checked")
                }
                _ => {()}
            }
        }
        None => {
            println!("No action argument. Use --help for more information.");
        }
    }

    Ok(())
}
