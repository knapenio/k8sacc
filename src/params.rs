use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Parameters(HashMap<String, String>);

impl Parameters {
    /// Get the value for a given parameter.
    pub fn get(&self, param: &str) -> Result<&String> {
        self.0
            .get(param)
            .ok_or_else(|| Error::MissingParameter(param.to_owned()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct DigitalOceanParameters {
    pub cluster: String,
    pub context: Option<String>,
}

#[derive(Serialize, Deserialize)]
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
