use anyhow::Result;
use clap::{Parser, Subcommand};
use modnote::{crud::notebook::create_notebook, db::set_db_options};
use sea_orm::Database;
use tracing_subscriber;

// A template for Rust CLI applications
#[derive(Parser, Debug)]
#[command(name = "modnote")]
#[command(version, about, long_about = None)]
struct Cli {
    // Subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // crud action args: NEW (create)
    New {
        #[command(subcommand)]
        subcommands: Option<Subcommands>,
    },
    // crud action args: GET (read)
    Get {
        #[command(subcommand)]
        subcommands: Option<Subcommands>,
    },
    // crud action args: UP (update)
    Up {
        #[command(subcommand)]
        subcommands: Option<Subcommands>,
    },
    // crud action args: DEL (delete)
    Del {
        #[command(subcommand)]
        subcommands: Option<Subcommands>,
    },
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    Notebook {
        #[arg(short, long, help = "Name of notebook")]
        name: String,

        #[arg(short, long, help = "Description of notebook")]
        desc: String,
    },
    Note {
        #[arg(short, long, help = "Name of note")]
        name: String,

        #[arg(short, long, help = "Content of note")]
        content: String,
    },
    Tag {
        #[arg(short, long, help = "Name of tag")]
        name: String,
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
        // Parse "New" Command
        Some(Commands::New { subcommands }) => match subcommands.as_ref().unwrap() {
            Subcommands::Notebook { name, desc } => {
                create_notebook(db, name, desc).await?;
            }
            Subcommands::Note { name, content } => {
                println!("Note: {}, {}", name, content)
            }
            Subcommands::Tag { name } => {
                println!("Tag: {}", name)
            }
        },
        // Parse "Get" Command
        Some(Commands::Get { subcommands }) => match subcommands.as_ref().unwrap() {
            Subcommands::Notebook { name, desc } => {
                create_notebook(db, name, desc).await?;
            }
            Subcommands::Note { name, content } => {
                println!("Note: {}, {}", name, content)
            }
            Subcommands::Tag { name } => {
                println!("Tag: {}", name)
            }
        },
        // Parse "Up" Command
        Some(Commands::Up { subcommands }) => match subcommands.as_ref().unwrap() {
            Subcommands::Notebook { name, desc } => {
                create_notebook(db, name, desc).await?;
            }
            Subcommands::Note { name, content } => {
                println!("Note: {}, {}", name, content)
            }
            Subcommands::Tag { name } => {
                println!("Tag: {}", name)
            }
        },
        // Parse "Del" Command
        Some(Commands::Del { subcommands }) => match subcommands.as_ref().unwrap() {
            Subcommands::Notebook { name, desc } => {
                create_notebook(db, name, desc).await?;
            }
            Subcommands::Note { name, content } => {
                println!("Note: {}, {}", name, content)
            }
            Subcommands::Tag { name } => {
                println!("Tag: {}", name)
            }
        },
        None => {
            println!("No action argument. Use --help for more information.");
        }
    }

    Ok(())
}
