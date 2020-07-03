extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use std::default::Default;
use std::string::String;

use ansi_term::Style;
use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use rcdom::{Handle, NodeData, RcDom};
use regex::Regex;

use crate::display::display;

const INDENT_SIZE: usize = 0;

pub fn walk(indent: usize, handle: &Handle, default_style: &Style) {
    let node = handle;

    let next_style = display(indent, node, default_style);

    for child in node.children.borrow().iter() {
        match child.data {
            NodeData::Text { ref contents } => {
                let re = Regex::new(r"^\s*\n?$").unwrap();
                if re.is_match(&contents.borrow()) {
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
        walk(indent + INDENT_SIZE, child, &next_style);
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
