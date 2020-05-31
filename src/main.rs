extern crate html5ever;
extern crate reqwest;

use ossifrage::fetch::fetch_document;

#[tokio::main]
async fn main() {
    let response = fetch_document().await;
    println!("{:}", response.unwrap());
}
