use anyhow::Result;
use fantoccini::{Client, Locator, elements::Element};
pub async fn title(client: &Client) -> Result<String> {
    let title_el = client.wait().for_element(Locator::Css("title")).await?;
    let title = title_el.html(true).await?;

    Ok(title)
}

pub async fn command_input(client: &Client) -> Result<Element> {
    Ok(client
        .wait()
        .for_element(Locator::Css("#command-input"))
        .await?)
}

pub async fn number_previous_commands(client: &Client) -> Result<u32> {
    let previous_commands = client
        .wait()
        .for_element(Locator::Css("#previous-commands"))
        .await?;

    let child_element_count: u32 = previous_commands
        .prop("childElementCount")
        .await?
        .expect("expected there to be some value for childElementCount property")
        .parse()
        .expect("expected childElementCount to be a number");

    Ok(child_element_count)
}
