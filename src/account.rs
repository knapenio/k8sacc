use crate::{params::Parameters, provider::Provider, Error, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Accounts(Vec<Account>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub alias: String,
    pub provider: Provider,
    pub params: Parameters,
}

impl Account {
    pub fn activate(&self) -> Result<()> {
        self.provider.activate_account(self.params.clone())
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
