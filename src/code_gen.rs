use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::parse::{Node, NodeType};
use crate::lang_util;

fn gen_html_tag_pair(tag_text: &str) -> (String, String) {
    (format!("<{}>", tag_text), format!("</{}>", tag_text))
}

fn node_to_html(node: &Node, mut base: String) -> String {
    lazy_static! {
        static ref HTML_TAG_LOOKUP: HashMap<NodeType, (String, String)> = {
            let mut tag_map = HashMap::new();
            
            tag_map.insert(NodeType::Root, gen_html_tag_pair("html"));
            tag_map.insert(NodeType::Main, gen_html_tag_pair("body"));
            tag_map.insert(NodeType::Chapter, gen_html_tag_pair("h1"));
            tag_map.insert(NodeType::Section, gen_html_tag_pair("h2"));
            tag_map.insert(NodeType::Subsection, gen_html_tag_pair("h3"));
            tag_map.insert(NodeType::Text, gen_html_tag_pair("p"));

            tag_map
        };
    }

    let tag = HTML_TAG_LOOKUP.get(&node.node_type).unwrap();
    base += &tag.0;

    match node.data {
        Some(ref data) => base += data,
        None => for child in &node.children {
            base = node_to_html(child, base);
        }
    }

    base += &tag.1;
    base
}

pub fn ast_to_html(root: &Node) -> String {
    if root.node_type != NodeType::Root {
        lang_util::error("tried converting non-root ast node to html");
    }

    lang_util::log("code generation complete");
    node_to_html(root, String::new())
}
