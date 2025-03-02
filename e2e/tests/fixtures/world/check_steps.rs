use crate::fixtures::{check, find, world::AppWorld};
use anyhow::{Ok, Result};
use cucumber::then;

#[then(expr = "{string} is in the title of the website")]
async fn is_in_the_title_of_the_website(world: &mut AppWorld, title: String) -> Result<()> {
    let client = &world.client;

    let present = check::title(client, title).await?;

    Ok(())
}

#[then("the output from previous commands disappears")]
async fn output_from_previous_commands_disappears(world: &mut AppWorld) -> Result<()> {
    let client = &world.client;
    let num_previous_commands = find::number_previous_commands(client).await?;
    assert_eq!(num_previous_commands, 0);
    Ok(())
}
