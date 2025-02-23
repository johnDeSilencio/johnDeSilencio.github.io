use crate::fixtures::{action, world::AppWorld};
use anyhow::{Ok, Result};
use cucumber::when;
use tokio::time::{Duration, sleep};

#[when("I open the website")]
async fn i_open_the_website(world: &mut AppWorld) -> Result<()> {
    let client = &world.client;
    action::goto_path(client, "").await?;

    // Sleep long enough for the website to load properly
    sleep(Duration::from_millis(1000)).await;

    Ok(())
}
