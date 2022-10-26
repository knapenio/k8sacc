use crate::{params::*, Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Provider {
    /// DigitalOcean
    #[serde(rename = "do")]
    DigitalOcean,
    /// Amazon EKS
    #[serde(rename = "eks")]
    Eks,
}

impl Provider {
    pub fn activate_account(&self, params: Parameters) -> Result<()> {
        use std::process::Command;

        let mut command: Command = match self {
            Self::DigitalOcean => {
                let params: DigitalOceanParameters = params.try_into()?;
                // doctl kubernetes cluster kubeconfig save <cluster> --context <context>
                let mut command = Command::new("doctl");
                command
                    .arg("kubernetes")
                    .arg("cluster")
                    .arg("kubeconfig")
                    .arg("save")
                    .arg(params.cluster);

                if let Some(context) = params.context {
                    command.args(["--context", &context]);
                }

                command
            }
            Self::Eks => {
                let params: EksParameters = params.try_into()?;
                // aws eks --region <region> update-kubeconfig --name <name> --region <region> --profile <profile>
                let mut command = Command::new("aws");
                command
                    .arg("eks")
                    .arg("update-kubeconfig")
                    .args(["--name", &params.name]);

                if let Some(region) = params.region {
                    command.args(["--region", &region]);
                }

                if let Some(profile) = params.profile {
                    command.args(["--profile", &profile]);
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
