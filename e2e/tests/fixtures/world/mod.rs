pub mod action_steps;
pub mod check_steps;

use anyhow::Result;
use cucumber::World;
use fantoccini::{error::NewSessionError, wd::Capabilities, Client, ClientBuilder};
use serde_json::{json, Map, Value};

pub const HOST: &str = "http://127.0.0.1:8080";

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct AppWorld {
    pub client: Client,
}

impl AppWorld {
    async fn new() -> Result<Self, anyhow::Error> {
        let webdriver_client = build_client().await?;

        Ok(Self {
            client: webdriver_client,
        })
    }
}

async fn build_client() -> Result<Client> {
    let caps = create_capabilities()?;

    let client = ClientBuilder::native()
        .capabilities(caps)
        .connect("http://127.0.0.1:4444")
        .await?;

    Ok(client)
}

fn create_capabilities() -> Result<Map<String, Value>> {
    let capabilities = json!({
        "moz:firefoxOptions": {
            "args": ["-headless"]
        }
    });

    let cap_map = capabilities
        .as_object()
        .ok_or(anyhow::anyhow!("failed to parse capabilities JSON"))?
        .to_owned();

    Ok(cap_map)
}
