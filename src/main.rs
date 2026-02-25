use anyhow::Result;
use clap::{Parser, Subcommand};
use modnote::{
    crud::notebook::*,
    crud::note::*,
    db::set_db_options};
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
    Update {
        #[command(subcommand)]
        subcommands: Option<Subcommands>,
    },
    /// Delete notebook, note or tag
    Delete {
        #[command(subcommand)]
        subcommands: Option<Subcommands>,
        // #[arg(short, long, help = "Get all notebooks, notes, or tags")]
        // all: Option<bool>,
    },
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    /// Create a new notebook
    Notebook {
        /// name of the notebook
        #[arg(short, long, help = "Name of notebook")]
        name: Option<String>,
        /// description of the notebook
        #[arg(short, long, help = "Description of notebook")]
        desc: Option<String>,
    },
    /// Create a new note
    Note {
        /// title of the note
        #[arg(short, long, help = "Title of note")]
        title: Option<String>,

        /// content of the note
        #[arg(short, long, help = "Content of note")]
        content: Option<String>,
    },
    /// Create a new tag
    Tag {
        /// name of the tag
        #[arg(short, long, help = "Name of tag")]
        name: Option<String>,
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
                if name.is_none() || desc.is_none() {
                    println!("\nerror: name and description required to create new notebook. use --help for correct usage\n")
                } else {
                    let name = name.to_owned().unwrap();
                    let desc = desc.to_owned().unwrap();
                    create_notebook(db, name, desc).await?;
                    println!("Successfully created notebook");
                }
            }
            Subcommands::Note { title, content } => {
                if title.is_none() || content.is_none() {
                    println!("\nerror: title and content required to create new note. use --help for correct usage\n")
                } else {
                    let title = title.to_owned().unwrap();
                    let content = content.to_owned().unwrap();
                    create_note(db, title, content).await?;
                    println!("Successfully created note");
                }
            }
            Subcommands::Tag { name: _ } => {
                println!("Tag: ");
                todo!("Implement create tag functionality")
            }
        },

        // Parse "Get" Command
        Some(Commands::Get { subcommands }) => match subcommands.as_ref().unwrap() {
            Subcommands::Notebook { name, desc: _ } => {
                if name.is_none() {
                    let notebooks = get_all_notebooks(db).await?;
                    for notebook in notebooks {
                        println!("{:?}", notebook)
                    }
                } else {
                    let notebook =
                        get_notebook_by_name(db, name.to_owned().unwrap_or_default()).await?;
                    println!("{:?}", notebook)
                }
            }
            Subcommands::Note { title: _, content: _ } => {
                println!("Note: ");
                todo!("Implement get note functionality")
            }
            Subcommands::Tag { name: _ } => {
                println!("Tag: ");
                todo!("Implement get tag functionality")
            }
        },

        // Parse "Up" Command
        Some(Commands::Update { subcommands }) => match subcommands.as_ref().unwrap() {
            Subcommands::Notebook { name, desc: _ } => {
                update_notebook_by_name(db, name.to_owned()).await?;
            }
            Subcommands::Note { title: _, content: _ } => {
                println!("Note: ")
            }
            Subcommands::Tag { name: _ } => {
                println!("Tag: ")
            }
        },

        // // Parse "Del" Command
        Some(Commands::Delete { subcommands }) => match subcommands.as_ref().unwrap() {
            Subcommands::Notebook { name, desc: _ } => {
                if name.is_none() {
                    delete_all_notebooks(db).await?;
                    println!("Successfully deleted all notebooks");
                } else {
                    delete_notebook_by_name(db, name).await?;
                    println!(
                        "Successfully deleted notebook with name: {}",
                        name.as_ref().unwrap_or(&"".to_string())
                    );
                }
            }
            Subcommands::Note { title: _, content: _ } => {
                println!("Note: ");
                todo!("Implement delete note functionality")
            }
            Subcommands::Tag { name: _ } => {
                println!("Tag: ");
                todo!("Implement delete tag functionality")
            }
        },

        // Handle no command given
        None => {
            println!("No action argument. Use --help for more information.");
        }
    }

    Ok(())
}
