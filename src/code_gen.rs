use std::process;

use crate::parse::Node;

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

pub fn generate_html(root: &Node) -> String {
    match root {
        Node::Root(_) => (),
        _ => {
            error!("tried to generate html from non-root node");
            process::exit(-1);
        }
    }

    node_to_html(root, String::new())
}
