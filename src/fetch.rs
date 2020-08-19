extern crate reqwest;

use regex::Regex;

pub async fn fetch_document(url: &str) -> Result<String, reqwest::Error> {
    let mut target_url: String = url.to_string();
    let re = Regex::new(r"^https?://*").unwrap();
    if !re.is_match(url) {
        target_url = format!("{}{}", "http://", &url);
    }
    let res = reqwest::get(&target_url).await?;
    let body = res.text().await?;

    Ok(body)
}
