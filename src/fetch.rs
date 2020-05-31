extern crate reqwest;

const MANUAL_URL: &str = "http://docs.rs/";

pub async fn fetch_document() -> Result<String, reqwest::Error> {
    let res = reqwest::get(MANUAL_URL).await?;
    let body = res.text().await?;

    Ok(body)
}
