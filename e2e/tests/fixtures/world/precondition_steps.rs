use crate::fixtures::{action, find::command_input, world::AppWorld};
use anyhow::{Ok, Result};
use cucumber::given;
use fantoccini::key;

#[given("the user has already executed several commands")]
async fn execute_commands(world: &mut AppWorld) -> Result<()> {
    let client = &world.client;
    action::goto_path(client, "").await?;
    // Wait long enough for page to load
    let _ = tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let input_field = command_input(client).await?;

    input_field.send_keys("First command!").await?;
    input_field.send_keys(&key::Key::Enter).await?;

    let _ = tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    input_field.send_keys("Second command!").await?;
    input_field.send_keys(&key::Key::Enter).await?;

    let _ = tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    input_field.send_keys("Third command!").await?;
    input_field.send_keys(&key::Key::Enter).await?;

    let _ = tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(())
}
