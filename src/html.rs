use html5ever::driver::ParseOpts;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize};

pub fn parse_html(input: String) -> String {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut input.as_bytes())
        .unwrap();
    walk(&dom.document).join("\n")
}
fn walk(handle: &Handle) -> Vec<String> {
    let node = handle;
    let mut text: Vec<String> = vec![];
    match node.data {
        NodeData::Text { ref contents } => {
            if !contents.borrow().trim().is_empty() {
                text.push(contents.borrow().to_string());
            }
            text
        }

        // NodeData::Element {
        //     ref name,
        //     ref attrs,
        //     ..
        // } => {
        //     assert!(name.ns == ns!(html));
        //     print!("<{}", name.local);
        //     for attr in attrs.borrow().iter() {
        //         assert!(attr.name.ns == ns!());
        //         print!(" {}=\"{}\"", attr.name.local, attr.value);
        //     }
        //     println!(">");
        // }
        _ => {
            for child in node.children.borrow().iter() {
                text.append(&mut walk(&*child));
            }
            text
        }
    }
}
