use crate::fixtures::{check, world::AppWorld};
use anyhow::{Ok, Result};
use cucumber::then;

#[then(expr = "{string} is in the title of the website")]
async fn is_in_the_title_of_the_website(world: &mut AppWorld, title: String) -> Result<()> {
    let client = &world.client;

    let present = check::title(client, title).await?;

    Ok(())
}
