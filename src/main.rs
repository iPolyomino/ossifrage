use std::env;
use std::process;

use ossifrage::fetch::fetch_document;
use ossifrage::parse::{html2dom, walk};
use ansi_term::Style;

#[tokio::main]
async fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("please set url in argument");
        process::exit(1);
    }
    let url : &str = &args[1];
    match fetch_document(&url).await {
        Ok(response) => {
            let dom = html2dom(&response);
            walk(0, &dom.document, &Style::new());
        },
        Err(_) => {
            println!("cannot access to {}", url);
            process::exit(1);
        }
    }
}
