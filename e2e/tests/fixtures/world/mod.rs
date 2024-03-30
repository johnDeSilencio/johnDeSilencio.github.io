pub mod action_steps;
pub mod check_steps;

use anyhow::Result;
use cucumber::World;
use fantoccini::{error::NewSessionError, wd::Capabilities, Client, ClientBuilder};

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

async fn build_client() -> Result<Client, NewSessionError> {
    let mut cap = Capabilities::new();

    let client = ClientBuilder::native()
        .capabilities(cap)
        .connect("http://127.0.0.1:4444")
        .await?;

    Ok(client)
}
