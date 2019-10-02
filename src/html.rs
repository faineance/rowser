use html5ever::driver::ParseOpts;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize};

pub struct Block {
    pub class: BlockClass,
    pub content: Vec<Span>,
}

pub enum Span {
    Text { class: SpanClass, content: String },
}

pub enum BlockClass {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Paragraph,
    Pre,
}

pub enum SpanClass {
    Bold,
    Italic,
    // Link,
    Regular,
}

pub struct WalkState {
    bold: bool,
    italic: bool,
}
impl Default for WalkState {
    fn default() -> Self {
        WalkState {
            bold: false,
            italic: false,
        }
    }
}

fn walk(handle: &Handle, state: &WalkState) -> (Vec<Block>) {
    let node = handle;
    match node.data {
        // NodeData::Text { ref contents } => {
        //     if !contents.borrow().trim().is_empty() {
        //         return (vec!())
        //         state.blocks.push(contents.borrow().to_string());
        //     }
        //     text
        // }
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            match name.local {
                local_name!("b") if !state.bold => walk(
                    node,
                    &WalkState {
                        bold: true,
                        ..*state
                    },
                ),
                // _ => walk(node, state),
                _ => vec![],
            }
            // for attr in attrs.borrow().iter() {
            //     assert!(attr.name.ns == ns!());
            //     print!(" {}=\"{}\"", attr.name.local, attr.value);
            // }
            // println!(">");
            // return (
        }
        _ => {
            let mut blocks = vec![];
            for child in node.children.borrow().iter() {
                blocks.append(&mut walk(&*child, state));
            }
            blocks
        }
    }
}

pub fn parse_html(input: String) -> Vec<Block> {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut input.as_bytes())
        .unwrap();
    walk(&dom.document, &WalkState::default())
}
