use crate::fixtures::{action, find::command_input, world::AppWorld};
use anyhow::{Ok, Result};
use cucumber::when;
use fantoccini::key;
use tokio::time::{Duration, sleep};

#[when("I open the website")]
async fn i_open_the_website(world: &mut AppWorld) -> Result<()> {
    let client = &world.client;
    action::goto_path(client, "").await?;

    // Sleep long enough for the website to load properly
    sleep(Duration::from_millis(1000)).await;

    Ok(())
}

#[when("the user executes the command 'clear'")]
async fn execute_clear_command(world: &mut AppWorld) -> Result<()> {
    let client = &world.client;

    let input_field = command_input(client).await?;
    input_field.send_keys("clear").await?;
    input_field.send_keys(&key::Key::Return).await?;

    let _ = tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(())
}
