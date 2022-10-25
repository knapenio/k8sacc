use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Parameters(HashMap<String, String>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Accounts(Vec<Account>);

#[derive(Serialize, Deserialize, Debug)]
pub enum Provider {
    #[serde(rename = "do")]
    DigitalOcean,
    #[serde(rename = "eks")]
    EKS,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub alias: String,
    pub provider: Provider,
    pub params: Parameters,
}

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
    /// Path to the configuration file [default: ~/.awsacc]
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
            .join(".awsacc");
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

impl Account {
    pub fn activate(&self) -> Result<()> {
        self.provider.activate_account(&self.params)
    }
}

impl Parameters {
    pub fn get(&self, key: &str) -> Result<&String> {
        self.0
            .get(key)
            .ok_or_else(|| Error::MissingParameter(key.to_owned()))
    }
}

impl Accounts {
    pub fn parse<P>(path: P) -> Result<Accounts>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let accounts = serde_yaml::from_reader(BufReader::new(file))?;
        Ok(Accounts(accounts))
    }

    pub fn get(&self, alias: &str) -> Result<&Account> {
        self.0
            .iter()
            .find(|acc| acc.alias == alias)
            .ok_or_else(|| Error::UnknownAccount(alias.to_owned()))
    }

    pub fn sorted(mut self) -> Vec<Account> {
        self.0.sort_by(|lhs, rhs| lhs.alias.cmp(&rhs.alias));
        self.0
    }
}

impl Provider {
    pub fn activate_account(&self, params: &Parameters) -> Result<()> {
        use std::process::Command;

        let mut command: Command = match self {
            Self::DigitalOcean => {
                // doctl kubernetes cluster kubeconfig save <cluster>
                let cluster = params.get("cluster")?;
                let context = params.get("context").ok();
                let mut command = Command::new("doctl");
                command
                    .arg("kubernetes")
                    .arg("cluster")
                    .arg("kubeconfig")
                    .arg("save")
                    .arg(cluster);

                if let Some(context) = context {
                    command.args(["--context", context]);
                }

                command
            }
            Self::EKS => {
                // aws eks --region <region> update-kubeconfig --name <name> --profile <profile>
                let name = params.get("name")?;
                let region = params.get("region").ok();
                let profile = params.get("profile").ok();
                let mut command = Command::new("aws");
                command
                    .arg("eks")
                    .arg("update-kubeconfig")
                    .args(["--name", name]);

                if let Some(region) = region {
                    command.args(["--region", region]);
                    // command.arg("--region").arg(region);
                }

                if let Some(profile) = profile {
                    command.args(["--profile", profile]);
                }

                command
            }
        };

        println!("{:?}", command);
        let output = command
            .output()
            .map_err(|e| Error::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(Error::CommandFailed(
                String::from_utf8(output.stderr).unwrap_or_default(),
            ));
        }

        Ok(())
    }
}
