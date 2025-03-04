use super::world::HOST;
use anyhow::Result;
use fantoccini::Client;

pub async fn goto_path(client: &Client, path: &str) -> Result<()> {
    let url = format!("{}{}", HOST, path);
    client.goto(&url).await?;

    Ok(())
}
