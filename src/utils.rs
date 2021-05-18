use netc::Client;

use crate::errors::RssError;

pub async fn my_ip() -> Result<String, RssError> {
    let mut client = Client::builder()
        .get("https://api.ipify.org")
        .build()
        .await?;
    Ok(client.send().await?.text()?)
}
