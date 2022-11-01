use std::process;

use crate::parse::Node;
use crate::lazy_regex;
use crate::lang_util::FindRev;

fn node_to_html(node: &Node, base: String) -> String {
    let parental = |tag, children: &Vec<Box<Node>>, mut base| {
        base += &format!("<{}>", tag) as &str;
        for child in children.iter().map(|child| &*child) {
            base = node_to_html(child, base);
        }
        
        base += &format!("</{}>", tag) as &str;
        base
    };

    let with_data = |tag, data, mut base| {
        base += &format!("<{}>{}</{}>", tag, data, tag) as &str;
        base
    };

    let wrapped = |outer_tag, inner_tag, children: &Vec<Box<Node>>, mut base| {
        base += &format!("<{}>", outer_tag) as &str;
        for child in children {
            base += &format!("<{}>", inner_tag) as &str;
            base = node_to_html(child, base);
            base += &format!("</{}>", inner_tag) as &str;
        }
        
        base += &format!("</{}>", outer_tag) as &str;
        base
    };
    
    match node {
        Node::Root(ref children) => parental("html", children, base),
        Node::Contents(ref children) => parental("body", children, base),
        Node::Chapter(ref data) => with_data("h1", data, base),
        Node::Section(ref data) => with_data("h2", data, base),
        Node::Subsection(ref data) => with_data("h3", data, base),
        Node::Text(ref data) => with_data("p", data, base),
        Node::List(ref children) => wrapped("ul", "li", children, base),
        Node::Table(ref children) => parental("table", children, base),
        Node::Row(ref children) => wrapped("tr", "td", children, base),
    }
}

fn postprocess(html: &str) -> String {
    lazy_regex! {
        PROTECTED_SEQ = r"@#':\[;:[A-Z][A-Z0-9_]\]";
    }

    let mut html = html.to_string();
    for mat in PROTECTED_SEQ.find_rev(&html.clone()) {
        let prot_code = html[(mat.start() + 7)..(mat.start() + 9)].to_string();
        let replacement = match prot_code.as_ref() {
            "LB" => "{",
            "RB" => "}",
            "EC" => "]",
            "P_" => ".",
            "A_" => "@",
            
            // a user should never encode protected sequences manually.
            // if they do, and they make a mistake, this will quietly remove it.
            _ => "",
        };

        html.replace_range(mat.range(), replacement);
    }

    html
}

pub fn generate_html(root: &Node) -> String {
    match root {
        Node::Root(_) => (),
        _ => {
            error!("tried to generate html from non-root node");
            process::exit(-1);
        }
    }

    postprocess(&node_to_html(root, String::new()))
}
