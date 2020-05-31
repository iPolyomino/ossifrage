use ossifrage::fetch::fetch_document;
use ossifrage::parse::{html2dom, walk};
use ansi_term::Colour;

#[tokio::main]
async fn main() {
    let response = fetch_document().await;
    let html = response.unwrap();
    let dom = html2dom(&html);
    walk(0, &dom.document, &Colour::White);
}
