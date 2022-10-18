use logos::Logos;

use crate::lang_util;
use crate::lang_util::Token;

#[derive(Eq, PartialEq, Hash)]
pub enum NodeType {
    Root,
    Main,
    Chapter,
    Section,
    Subsection,
    Text,

    // only exists to allow certain things to be skipped in parsing.
    // this should *never* be used as the `node_type` for an actual `Node`.
    Other,
}

impl From<Token> for NodeType {
    fn from(tok: Token) -> Self {
        match tok {
            Token::Main => Self::Main,
            Token::Chapter => Self::Chapter,
            Token::Section => Self::Section,
            Token::Subsection => Self::Subsection,
            Token::Text => Self::Text,
            _ => Self::Other,
        }
    }
}

pub struct Node {
    pub node_type: NodeType,
    pub data: Option<String>,
    pub children: Vec<Box<Node>>,
}

impl Node {
    fn new(node_type: NodeType) -> Self {
        Self {
            node_type,
            data: None,
            children: Vec::new(),
        }
    }
    
    fn data(mut self, data: &str) -> Self {
        self.data = Some(data.to_string());
        self
    }

    fn children(mut self, children: Vec<Box<Node>>) -> Self {
        self.children = children;
        self
    }
}

fn parse_main(src: &str) -> Node {
    let mut lex = Token::lexer(src);
    let mut children = Vec::new();
    
    while let Some(tok) = lex.next() {
        let node_type = NodeType::from(tok);
        match node_type {
            NodeType::Chapter |
            NodeType::Section |
            NodeType::Subsection |
            NodeType::Text => {
                let data = lang_util::extract_arg(&mut lex, src);
                children.push(Box::new(Node::new(node_type).data(&data)));
            }
            NodeType::Other => continue,
            _ => lang_util::error("unexpected node type!"),
        }
    }

    lang_util::log("parsed main node");
    Node::new(NodeType::Main).children(children)
}

pub fn parse(src: &str) -> Node {
    let mut lex = Token::lexer(src);
    let mut children = Vec::new();

    while let Some(tok) = lex.next() {
        match NodeType::from(tok) {
            NodeType::Main => {
                let main_src = lang_util::extract_arg(&mut lex, src);
                children.push(Box::new(parse_main(&main_src)));
            }
            NodeType::Other => continue,
            _ => lang_util::error("unexpected node type!"),
        }
    }

    lang_util::log("parsing complete");
    Node::new(NodeType::Root).children(children)
}
