use std::env;
use std::io::{self, Write};
use std::process;

use ansi_term::Style;
use ossifrage::fetch::fetch_document;
use ossifrage::parse::{html2dom, walk};

#[tokio::main]
async fn main() {
    let mut url = String::new();

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        url = String::from(&args[1]);
    }
    if url.is_empty() {
        io::stdin().read_line(&mut url).unwrap();
    }

    if url.is_empty() {
        io::stdout()
            .write("please set url in argument".as_bytes())
            .unwrap();
        process::exit(1);
    }

    match fetch_document(&url).await {
        Ok(response) => {
            let dom = html2dom(&response);
            walk(0, &dom.document, &Style::new());
        }
        Err(_) => {
            println!("cannot access to {}", url);
            process::exit(1);
        }
    }
}
