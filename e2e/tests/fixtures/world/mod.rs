pub mod action_steps;
pub mod check_steps;

use anyhow::Result;
use cucumber::World;
use fantoccini::{Client, ClientBuilder, error::NewSessionError, wd::Capabilities};
use serde_json::{Map, Value, json};

pub const HOST: &str = "http://127.0.0.1:8080";
pub const HEADLESS_ENV_VAR: &str = "END_TO_END_TESTS_HEADLESS";

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct AppWorld {
    pub client: Client,
}

impl AppWorld {
    async fn new() -> Result<Self, anyhow::Error> {
        let headless = match std::env::var(HEADLESS_ENV_VAR)
            .unwrap_or(String::from(""))
            .as_str()
        {
            "true" => true,
            _ => false,
        };

        let webdriver_client = build_client(headless).await?;

        Ok(Self {
            client: webdriver_client,
        })
    }
}

async fn build_client(headless: bool) -> Result<Client> {
    let caps = create_capabilities(headless)?;

    let client = ClientBuilder::native()
        .capabilities(caps)
        .connect("http://127.0.0.1:4444")
        .await?;

    Ok(client)
}

fn create_capabilities(headless: bool) -> Result<Map<String, Value>> {
    let capabilities = match headless {
        true => {
            json!({
                "moz:firefoxOptions": {
                    "args": ["-headless"]
                }
            })
        }
        false => json!({}),
    };

    let cap_map = capabilities
        .as_object()
        .ok_or(anyhow::anyhow!("failed to parse capabilities JSON"))?
        .to_owned();

    Ok(cap_map)
}
