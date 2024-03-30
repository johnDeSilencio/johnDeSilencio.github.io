use anyhow::Result;
use fantoccini::{elements::Element, Client, Locator};

pub async fn title(client: &Client) -> Result<String> {
    let title_el = client.wait().for_element(Locator::Css("title")).await?;
    let title = title_el.html(true).await?;

    Ok(title)
}
