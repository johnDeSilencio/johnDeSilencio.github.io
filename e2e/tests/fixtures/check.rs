use crate::fixtures::find;

use anyhow::{Ok, Result};
use fantoccini::{Client, Locator};
// use pretty_assertions::assert_eq;

pub async fn title(client: &Client, text: impl Into<String>) -> Result<()> {
    let expected_title_text = text.into();
    let title_text = find::title(client).await?;

    assert_eq!(title_text, expected_title_text);

    Ok(())
}
