use html5ever::driver::ParseOpts;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize};

#[derive(Debug)]
pub struct Block {
    pub class: BlockClass,
    pub content: Vec<Span>,
}

#[derive(Debug)]
pub enum Span {
    Text { class: SpanClass, content: String },
}

#[derive(Debug)]
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

#[derive(Debug)]
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

fn walk(handle: &Handle, state: &WalkState) -> (Vec<Block>, Vec<Span>) {
    let node = handle;
    match node.data {
        NodeData::Text { ref contents } => {
            if !contents.borrow().trim().is_empty() {
                let class = match (state.bold, state.italic) {
                    (true, false) => SpanClass::Bold,
                    (false, true) => SpanClass::Italic,
                    (false, false) => SpanClass::Regular,
                    _ => unimplemented!(),
                };
                return (
                    vec![],
                    vec![Span::Text {
                        class: class,
                        content: (contents.borrow().to_string()),
                    }],
                );
            }
            return (vec![], vec![]);
        }
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            print!("{}", name.local);
            match name.local {
                local_name!("b") if !state.bold => walk(
                    node,
                    &WalkState {
                        bold: true,
                        ..*state
                    },
                ),
                local_name!("i") if !state.italic => walk(
                    node,
                    &WalkState {
                        italic: true,
                        ..*state
                    },
                ),
                local_name!("div") | local_name!("p") => {
                    return (new_block(node, state, BlockClass::Paragraph), vec![]);
                },
                local_name!("h1") => {
                    return (new_block(node, state, BlockClass::H1), vec![]);
                },
                local_name!("h2") => {
                    return (new_block(node, state, BlockClass::H2), vec![]);
                },
                local_name!("h3") => {
                    return (new_block(node, state, BlockClass::H3), vec![]);
                },
                _ => {
                    let mut blocks = (vec![], vec![]);
                    for child in node.children.borrow().iter() {
                        let (mut new_blocks, mut new_spans) = walk(&*child, state);
                        blocks.0.append(&mut new_blocks);
                        blocks.1.append(&mut new_spans);
                    }
                    blocks
                }
                // _ => (vec![], vec![]),
                // _ => walk(node, state),
            }
            // for attr in attrs.borrow().iter() {
            //     assert!(attr.name.ns == ns!());
            //     print!(" {}=\"{}\"", attr.name.local, attr.value);
            // }
            // println!(">");
        }
        _ => {
            let mut blocks = (vec![], vec![]);
            for child in node.children.borrow().iter() {
                let (mut new_blocks, mut new_spans) = walk(&*child, state);
                blocks.0.append(&mut new_blocks);
                blocks.1.append(&mut new_spans);
            }
            blocks
        }
    }
}
pub fn new_block(node: &Handle, state: &WalkState, class: BlockClass) -> Vec<Block> {
    let mut res = (vec![], vec![]);
    for child in node.children.borrow().iter() {
        let (mut new_blocks, mut new_spans) = walk(&*child, state);
        res.0.append(&mut new_blocks);
        res.1.append(&mut new_spans);
    }
    res.0.push(Block {
        content: res.1,
        class,
    });
    return res.0;
}
pub fn parse_html(input: String) -> Vec<Block> {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut input.as_bytes())
        .unwrap();
    walk(&dom.document, &WalkState::default()).0
}
