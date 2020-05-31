extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use std::default::Default;
use std::iter::repeat;
use std::string::String;

use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use rcdom::{Handle, NodeData, RcDom};

pub fn walk(indent: usize, handle: &Handle) {
    let node = handle;

    print!("{}", repeat(" ").take(indent).collect::<String>());

    match node.data {
        NodeData::Text { ref contents } => println!("{}", escape_default(&contents.borrow())),

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
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
                if tag_name == "script" || tag_name == "head" {
                    continue;
                }
            }
            _ => (),
        }
        walk(indent + 4, child);
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
