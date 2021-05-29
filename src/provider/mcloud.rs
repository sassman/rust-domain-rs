use crate::app_config::{app_config_path, save, AppConfig};
use crate::interact::{user_name, user_secret};
use crate::Result;

use super::DomainServiceProvider;

use anyhow::bail;
use reqwest::blocking::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

pub const PROVIDER: &str = "mcloud";
const API_BASE_URL: &str = "https://portal.mcloud.rs/api";

pub struct MCloud;
impl MCloud {
    fn post(&self, url: &str, config: AppConfig) -> Result<RequestBuilder> {
        Ok(Client::builder()
            .build()?
            .post(url)
            .header("Authorization", config.token))
    }
}

impl DomainServiceProvider for MCloud {
    fn signup(&self) -> Result<()> {
        const SIGNUP_URL: &str = "https://portal.mcloud.rs/signup&languagechange=English";
        println!(
            r#"💡 Before we get started, please note that the domain service provider (ISP) 
   is an external standalone entity. *rust-domain* acts as a proxy betwen you and them.
   
   All providers must be accredited registras with the RNIDS, read more here:
   https://www.rnids.rs/en/registrars/list-accredited-registrars.
---------------------------------------------------------------------------------------
"#
        );
        println!("To proceed, please visit the URL and create an account:");
        println!("{}", SIGNUP_URL);
        println!();
        println!("Once you completed the process you need to enter your credentials below (for your basic-auth token)");
        let username = user_name()?;
        let password = user_secret()?;

        let path = &app_config_path()?;
        println!("Writing config file: {}", path.as_path().display());
        let config = AppConfig {
            provider: PROVIDER.to_owned(),
            token: format!(
                "Basic {}",
                base64::encode(format!("{}:{}", username, password))
            ),
        };
        save(&config)
    }

    fn check_domain(&self, domain: &str) -> Result<bool> {
        let config = AppConfig::load()?;
        let url = format!("{base_url}/domain/lookup", base_url = API_BASE_URL);
        let params = [("name", domain)];
        let response = self.post(url.as_str(), config)?.form(&params).send()?;

        let resp: CheckDomainPayload = response.json()?;
        match resp {
            CheckDomainPayload::Ok { available, .. } => Ok(available),
            CheckDomainPayload::Err { .. } => {
                bail!("Something went wrong, check if that was a valid domain")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum CheckDomainPayload {
    Ok {
        success: bool,
        name: String,
        available: bool,
    },
    Err {
        success: bool,
    },
}
