extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use std::default::Default;
use std::iter::repeat;
use std::string::String;

use ansi_term::{Colour, Style};
use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use rcdom::{Handle, NodeData, RcDom};

const INDENT_SIZE: usize = 0;
const DISPLAY_TAGS: bool = false;

pub fn walk(indent: usize, handle: &Handle, default_style: &Style) {
    let mut style: Style = *default_style;
    let node = handle;

    print!("{}", repeat(" ").take(indent).collect::<String>());

    match node.data {
        NodeData::Text { ref contents } => {
            let line: &str = &contents.borrow();
            println!("{}", style.paint(line));
        }

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            if DISPLAY_TAGS {
                print!("<{}", name.local);
                for attr in attrs.borrow().iter() {
                    print!(" {}=\"{}\"", attr.name.local, attr.value);
                }
                println!(">");
            }
            let tag_name = name.local.to_string();
            match &*tag_name {
                "h1" => style = Colour::RGB(255, 255, 0).bold(),
                "h2" => style = Colour::RGB(255, 255, 50).bold(),
                "h3" => style = Colour::RGB(255, 255, 100).bold(),
                "h4" => style = Colour::RGB(255, 255, 150).bold(),
                "h5" => style = Colour::RGB(255, 255, 200).bold(),
                "code" => style = Colour::RGB(200, 200, 200).italic(),
                _ => (),
            }
        }
        _ => {}
    }

    for child in node.children.borrow().iter() {
        match child.data {
            NodeData::Text { ref contents } => {
                if contents.borrow().to_string() == "\n" {
                    continue;
                }
            }

            NodeData::Element { ref name, .. } => {
                let tag_name = name.local.to_string();
                if tag_name == "script" || tag_name == "head" || tag_name == "input" {
                    continue;
                }
            }
            _ => (),
        }
        walk(indent + INDENT_SIZE, child, &style);
    }
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

pub fn html2dom(html: &str) -> RcDom {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    return dom;
}
