use crate::{params::Parameters, provider::Provider, Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Accounts(Vec<Account>);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
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
    fn from_reader<R>(rdr: R) -> Result<Accounts>
    where
        R: BufRead,
    {
        let accounts = serde_yaml::from_reader(rdr)?;
        Ok(Accounts(accounts))
    }

    pub fn parse<P>(path: P) -> Result<Accounts>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Accounts::from_reader(BufReader::new(file))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accounts() {
        assert!(Accounts::from_reader("".as_bytes()).unwrap().is_empty());
        assert!(Accounts::from_reader("-".as_bytes()).is_err());

        assert_eq!(
            Accounts::from_reader(
                "
            -
                alias: do-alpha
                provider: do
                params:
                    a: b
            -
                alias: eks-beta
                provider: eks
                params:
                    true: no
                    false: yes
            "
                .as_bytes()
            )
            .unwrap()
            .0,
            vec![
                Account {
                    alias: "do-alpha".to_owned(),
                    provider: Provider::DigitalOcean,
                    params: Parameters::from_iter([("a".to_owned(), "b".to_owned())])
                },
                Account {
                    alias: "eks-beta".to_owned(),
                    provider: Provider::EKS,
                    params: Parameters::from_iter([
                        ("true".to_owned(), "no".to_owned()),
                        ("false".to_owned(), "yes".to_owned())
                    ])
                }
            ]
        );
    }

    #[test]
    fn get_account() {
        let accounts = Accounts(vec![Account {
            alias: "john".to_owned(),
            provider: Provider::DigitalOcean,
            params: Default::default(),
        }]);

        assert_eq!(
            accounts.get("john").unwrap(),
            &Account {
                alias: "john".to_owned(),
                provider: Provider::DigitalOcean,
                params: Default::default(),
            }
        );
        assert!(accounts.get("doe").is_err());
    }

    #[test]
    fn accounts_sorted() {
        let accounts = Accounts(vec![
            Account {
                alias: "john".to_owned(),
                provider: Provider::DigitalOcean,
                params: Default::default(),
            },
            Account {
                alias: "lucy".to_owned(),
                provider: Provider::EKS,
                params: Default::default(),
            },
            Account {
                alias: "albert".to_owned(),
                provider: Provider::DigitalOcean,
                params: Default::default(),
            },
        ]);

        assert_eq!(
            accounts.sorted(),
            vec![
                Account {
                    alias: "albert".to_owned(),
                    provider: Provider::DigitalOcean,
                    params: Default::default(),
                },
                Account {
                    alias: "john".to_owned(),
                    provider: Provider::DigitalOcean,
                    params: Default::default(),
                },
                Account {
                    alias: "lucy".to_owned(),
                    provider: Provider::EKS,
                    params: Default::default(),
                },
            ]
        );
    }
}
