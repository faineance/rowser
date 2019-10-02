

use html5ever::driver::ParseOpts;
use html5ever::rcdom::{RcDom, Node, NodeData};
use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize};


/// Consumes a string that contains HTML5 tags and spits out a Vec<String>
/// containing the text content inside the tags in a pre-order manner.
///
/// Usage:
/// ```
/// let input = "<html>Hello World!</html>".to_owned();
/// let output = strip_html_tags(input);
/// assert_eq!(output, "Hello World!".to_owned()");
/// ```
pub fn strip_html_tags(input: &str) -> Vec<String> {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut input.as_bytes()).unwrap();

    let ref doc = *dom.document;
    get_text(doc)
}

/// Helper function to return text in text nodes in pre-order traversal.
fn get_text(element: &Node) -> Vec<String> {
    match element.data {
        NodeData::Text {contents: ref s }=> {
            let mut text = vec!((&**s).to_owned());
            for child in element.children.borrow() {
                text.append(&mut get_text(&*child.borrow()));
            }
            text
        }
        _ => {
            let mut text = vec!();
            for child in &element.children {
                text.append(&mut get_text(&*child.borrow()));
            }
            text
        }
    }
}
