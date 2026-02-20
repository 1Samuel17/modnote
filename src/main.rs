use anyhow::Result;
use clap::{Parser, Subcommand};
use modnote::{
    crud::notebook::create_notebook, crud::notebook::delete_notebook, db::set_db_options,
};
use sea_orm::Database;

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
    /// Create a new notebook, note or tag
    New {
        #[command(subcommand)]
        subcommands: Option<Subcommands>,
    },
    /// Get notebook(s), note(s), or tag(s)
    Get {
        #[command(subcommand)]
        subcommands: Option<Subcommands>,
    },
    /// Update notebook, note, or tag
    Up {
        #[command(subcommand)]
        subcommands: Option<Subcommands>,
    },
    /// Delete notebook, note or tag
    Del {
        #[command(subcommand)]
        subcommands: Option<Subcommands>,
    },
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    /// Create a new notebook
    Notebook {
        /// name of the notebook
        #[arg(short, long, help = "Name of notebook")]
        name: String,
        /// description of the notebook
        #[arg(short, long, help = "Description of notebook", default_value = "")]
        desc: String,
    },
    /// Create a new note
    Note {
        /// name of the note
        #[arg(short, long, help = "Name of note")]
        name: String,

        /// content of the note
        #[arg(short, long, help = "Content of note", default_value = "")]
        content: String,
    },
    /// Create a new tag
    Tag {
        /// name of the tag
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
                println!("Note: {}, {}", name, desc)
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
                println!("Note: {}, {}", name, desc)
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
            Subcommands::Notebook { name, desc: _ } => {
                delete_notebook(db, name).await?;
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
