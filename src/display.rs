extern crate markup5ever_rcdom as rcdom;

use std::iter::repeat;

use ansi_term::{Colour, Style};
use rcdom::{Handle, NodeData};

const DISPLAY_TAGS: bool = false;

pub fn display(indent: usize, node: &Handle, default_style: &Style) -> Style {
    let mut style: Style = *default_style;
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
                // Content sectioning
                "h1" => style = Colour::RGB(255, 255, 0).bold(),
                "h2" => style = Colour::RGB(255, 255, 50).bold(),
                "h3" => style = Colour::RGB(255, 255, 100).bold(),
                "h4" => style = Colour::RGB(255, 255, 150).bold(),
                "h5" => style = Colour::RGB(255, 255, 200).bold(),
                "h6" => style = Colour::RGB(255, 255, 250).bold(),

                // Inline text semantics
                "b" => style = style.bold(),
                "em" => style = style.italic(),
                "i" => style = style.italic(),
                "s" => style = style.strikethrough(),
                "strong" => style = style.bold(),
                "u" => style = style.underline(),
                _ => (),
            }
        }
        _ => {}
    }
    style
}
