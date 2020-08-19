mod event;

use std::env;
use std::io::{self, Write};
use std::process;

use ansi_term::Style;
use tui::backend::CrosstermBackend;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

use crate::event::read_char;
use ossifrage::fetch::fetch_document;
use ossifrage::parse::{html2dom, walk};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    clear_terminal()?;

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

    draw_terminal(&url).await?;

    Ok(())
}

fn clear_terminal() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()
}

async fn draw_terminal(url: &str) -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    loop {
        let lines = vec![
            Spans::from(vec![Span::raw("hello world!")]),
            Spans::from(vec![Span::raw("hello world?")]),
        ];
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("ossifrage").borders(Borders::ALL);
            let paragraph = Paragraph::new(lines).block(block);
            f.render_widget(paragraph, size);
        })?;

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

        if read_char().unwrap() == 'q' {
            break;
        }
    }

    Ok(())
}
