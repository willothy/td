use anyhow::{anyhow as bruh, Result};
use clap::{Parser, Subcommand};
use rustbreak::deser::Bincode;
use rustbreak::PathDatabase;

type Db = PathDatabase<Vec<String>, Bincode>;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    #[clap(about = "Add a todo", alias = "new", alias = "a")]
    Add { todo: String },
    #[clap(about = "List all todos", alias = "l", alias = "ls")]
    List,
    #[clap(about = "List all todos (for .*rc)")]
    Init,
    #[clap(about = "Remove a todo", alias = "r", alias = "rm")]
    Remove { index: usize },
}

fn main() -> Result<()> {
    let config = dirs::config_dir().ok_or_else(|| bruh!("Could not find config dir"))?;
    let db_path = config.join("todos.db");
    let db = Db::load_from_path_or_default(db_path)?;

    match Cli::parse().cmd.unwrap_or(Cmd::List) {
        Cmd::Add { todo } => {
            db.write(|db| {
                db.push(todo);
            })?;
            db.save()?;
        }
        Cmd::List | Cmd::Init => {
            db.read(|db| {
                for (i, todo) in db.iter().enumerate() {
                    println!("{}. {}", i, todo);
                }
            })?;
        }
        Cmd::Remove { index } => {
            db.write(|db| {
                db.remove(index);
            })?;
            db.save()?;
        }
    }

    Ok(())
}
