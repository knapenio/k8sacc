mod account;
mod params;
mod provider;

use account::*;
use clap::{Parser, Subcommand};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    #[error("Invalid configuration file")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Unknown account {0}")]
    UnknownAccount(String),
    #[error("Missing parameter {0}")]
    MissingParameter(String),
    #[error("Command failed: {0}")]
    CommandFailed(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Print list of available accounts
    List,
    /// Activate a given account
    Activate { alias: String },
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
    /// Path to the configuration file [default: ~/.k8sacc]
    #[arg(short, long)]
    config: Option<String>,
}

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    let accounts = if let Some(path) = cli.config {
        Accounts::parse(path)?
    } else {
        let path = directories::UserDirs::new()
            .unwrap()
            .home_dir()
            .join(".k8sacc");
        Accounts::parse(path)?
    };

    match cli.command {
        Command::List => {
            println!("Available accounts:");

            for account in accounts.sorted() {
                println!("- {} ({:?})", account.alias, account.provider);
            }

            Ok(())
        }
        Command::Activate { alias } => {
            let account = accounts.get(&alias)?;
            account.activate()
        }
    }
}
