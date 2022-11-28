use std::process;
use std::fmt::{Display, Debug, Formatter};
use std::fmt;

use logos::{Logos, Lexer};

use crate::lang_util;

#[derive(Logos, PartialEq, Clone, Copy)]
enum Token {
    #[token("contents")]
    Contents,

    #[token("chapter")]
    Chapter,

    #[token("section")]
    Section,

    #[token("subsection")]
    Subsection,

    #[token("text")]
    Text,

    #[token("list")]
    List,

    #[token("ordered_list")]
    OrderedList,

    #[token("table")]
    Table,

    #[token("row")]
    Row,

    #[token("{")]
    BlockStart,

    #[token("}")]
    BlockEnd,

    #[error]
    #[regex(r"\s+", logos::skip)]
    Error,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let text = match self {
            Self::Contents => "contents",
            Self::Chapter => "chapter",
            Self::Section => "section",
            Self::Subsection => "subsection",
            Self::Text => "text",
            Self::List => "list",
            Self::OrderedList => "ordered list",
            Self::Table => "table",
            Self::Row => "table row",
            Self::BlockStart => "block start",
            Self::BlockEnd => "block end",
            _ => "other",
        };

        write!(f, "{}", text)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub enum Node {
    Root(Vec<Box<Node>>),
    Contents(Vec<Box<Node>>),
    Chapter(String),
    Section(String),
    Subsection(String),
    Text(String),
    List(Vec<Box<Node>>),
    OrderedList(Vec<Box<Node>>),
    Table(Vec<Box<Node>>),
    Row(Vec<Box<Node>>),
}

macro_rules! parsing_rules {
    (($t:expr, $lex:expr, $file:expr, $src:expr); $($i:ident => $e:expr,)*) => {
        match $t {
            $(Token::$i => $e,)*
            _ => {
                let ex = vec![$(Token::$i,)*];
                let err_msg = format!("expected one of {:?}, found {}", ex, $t);
                error!($file, lang_util::current_line($src, $lex), err_msg);
                process::exit(-1);
            }
        }
    };
}

macro_rules! textual_extract_parse {
    ($fname:ident, $node_type:ident) => {
        fn $fname(file_path: &str, src: &str, lex: &mut Lexer<Token>) -> Node {
            let text = lang_util::extract_arg(
                file_path,
                src,
                lex,
                Token::BlockStart,
                Token::BlockEnd,
            );

            Node::$node_type(text)
        }
    };
}

macro_rules! layer_add_parse {
    ($fname:ident, $node_type:ident) => {
        fn $fname(file_path: &str, src: &str, lex: &mut Lexer<Token>) -> Node {
            let mut children = Vec::new();
            let mut add_child = |child| children.push(Box::new(child));
            lang_util::expect_tok(file_path, src, lex, Token::BlockStart);
            while let Some(tok) = lex.next(){
                parsing_rules! {
                    (tok, lex, file_path, src);
                    Chapter => add_child(chapter(file_path, src, lex)),
                    Section => add_child(section(file_path, src, lex)),
                    Subsection => add_child(subsection(file_path, src, lex)),
                    Text => add_child(text(file_path, src, lex)),
                    List => add_child(list(file_path, src, lex)),
                    OrderedList =>add_child(ordered_list(file_path, src, lex)),
                    Table => add_child(table(file_path, src, lex)),
                    BlockEnd => break,
                }
            }

            Node::$node_type(children)
        }
    };
}

textual_extract_parse!(chapter, Chapter);
textual_extract_parse!(section, Section);
textual_extract_parse!(subsection, Subsection);
textual_extract_parse!(text, Text);

layer_add_parse!(list, List);
layer_add_parse!(ordered_list, OrderedList);
layer_add_parse!(row, Row);
layer_add_parse!(contents, Contents);

fn table(file_path: &str, src: &str, lex: &mut Lexer<Token>) -> Node {
    let mut children = Vec::new();
    let mut add_child = |child| children.push(Box::new(child));
    lang_util::expect_tok(file_path, src, lex, Token::BlockStart);
    while let Some(tok) = lex.next(){
        parsing_rules! {
            (tok, lex, file_path, src);
            Row => add_child(row(file_path, src, lex)),
            BlockEnd => break,
        }
    }

    Node::Table(children)
}

pub fn parse(file_path: &str, src: &str) -> Node {
    let mut children = Vec::new();
    let mut lex = Token::lexer(src);
    let mut add_child = |child| children.push(Box::new(child));
    while let Some(tok) = lex.next() {
        parsing_rules! {
            (tok, &lex, file_path, src);
            Contents => add_child(contents(file_path, src, &mut lex)),
        }
    }

    Node::Root(children)
}
