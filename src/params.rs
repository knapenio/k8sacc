use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
#[serde(transparent)]
pub struct Parameters(HashMap<String, String>);

impl Parameters {
    #[cfg(test)]
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (String, String)>,
    {
        Parameters(HashMap::from_iter(iter))
    }

    /// Get the value for a given parameter.
    pub fn get(&self, param: &str) -> Result<&String> {
        self.0
            .get(param)
            .ok_or_else(|| Error::MissingParameter(param.to_owned()))
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct DigitalOceanParameters {
    pub cluster: String,
    pub context: Option<String>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct EksParameters {
    pub name: String,
    pub region: Option<String>,
    pub profile: Option<String>,
}

impl TryFrom<Parameters> for DigitalOceanParameters {
    type Error = Error;

    fn try_from(params: Parameters) -> std::result::Result<Self, Self::Error> {
        let cluster = params.get("cluster").cloned()?;
        let context = params.get("context").ok().cloned();
        Ok(DigitalOceanParameters { cluster, context })
    }
}

impl TryFrom<Parameters> for EksParameters {
    type Error = Error;

    fn try_from(params: Parameters) -> std::result::Result<Self, Self::Error> {
        let name = params.get("name").cloned()?;
        let region = params.get("region").cloned().ok();
        let profile = params.get("profile").cloned().ok();
        Ok(EksParameters {
            name,
            region,
            profile,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_iter() {
        assert_eq!(
            Parameters::from_iter([
                ("one".to_owned(), "1".to_owned()),
                ("six".to_owned(), "666666".to_owned()),
                ("four".to_owned(), "4444".to_owned()),
            ]),
            Parameters(HashMap::from_iter([
                ("one".to_owned(), "1".to_owned()),
                ("six".to_owned(), "666666".to_owned()),
                ("four".to_owned(), "4444".to_owned()),
            ]))
        );
    }

    #[test]
    fn get_param() {
        let params = Parameters::from_iter([
            ("one".to_owned(), "1".to_owned()),
            ("two".to_owned(), "22".to_owned()),
            ("three".to_owned(), "333".to_owned()),
        ]);

        assert_eq!(params.get("one").unwrap(), "1");
        assert_eq!(params.get("two").unwrap(), "22");
        assert_eq!(params.get("three").unwrap(), "333");

        assert!(params.get("1").is_err());
        assert!(params.get("four").is_err());
    }

    #[test]
    fn into_do_params() {
        assert!(DigitalOceanParameters::try_from(Parameters::from_iter([(
            "test".to_owned(),
            "".to_owned()
        )]))
        .is_err());

        assert_eq!(
            DigitalOceanParameters::try_from(Parameters::from_iter([(
                "cluster".to_owned(),
                "test".to_owned()
            )]))
            .unwrap(),
            DigitalOceanParameters {
                cluster: "test".to_owned(),
                context: None
            }
        );

        assert_eq!(
            DigitalOceanParameters::try_from(Parameters::from_iter([
                ("cluster".to_owned(), "test".to_owned()),
                ("context".to_owned(), "please".to_owned())
            ]))
            .unwrap(),
            DigitalOceanParameters {
                cluster: "test".to_owned(),
                context: Some("please".to_owned())
            }
        );
    }

    #[test]
    fn into_eks_params() {
        assert!(EksParameters::try_from(Parameters::from_iter([(
            "test".to_owned(),
            "".to_owned()
        )]))
        .is_err());

        assert_eq!(
            EksParameters::try_from(Parameters::from_iter([(
                "name".to_owned(),
                "test".to_owned()
            )]))
            .unwrap(),
            EksParameters {
                name: "test".to_owned(),
                region: None,
                profile: None
            }
        );

        assert_eq!(
            EksParameters::try_from(Parameters::from_iter([
                ("name".to_owned(), "johndoe".to_owned()),
                ("region".to_owned(), "eu-central-1".to_owned()),
                ("profile".to_owned(), "test".to_owned())
            ]))
            .unwrap(),
            EksParameters {
                name: "johndoe".to_owned(),
                region: Some("eu-central-1".to_owned()),
                profile: Some("test".to_owned())
            }
        );
    }
}
