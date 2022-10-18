use std::process;
use std::fmt::{Formatter, Display};
use std::fmt;

use regex::{Regex, Match};
use colored::Colorize;
use logos::{Logos, Lexer};

pub trait FindRev {
    fn find_rev<'a>(&self, text: &'a str) -> Vec<Match<'a>>;
}

impl FindRev for Regex {
    fn find_rev<'a>(&self, text: &'a str) -> Vec<Match<'a>> {
        self
            .find_iter(text)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
    }
}

// nice syntax for writing `lazy_static!` declarations which only contain
// regexes, instead of needing to write `Regex::new(...).unwrap()` every time.
#[macro_export]
macro_rules! lazy_regex {
    ($($n:ident = $v:literal;)*) => {
        lazy_static::lazy_static! {
            $(static ref $n: regex::Regex = regex::Regex::new($v).unwrap();)*
        }
    }
}

pub fn collapse_whitespace(src: &str) -> String {
    lazy_regex! {
        WHITESPACE = r"\s+";
    }

    WHITESPACE.replace_all(src, " ").to_string()
}

pub enum CommentType {
    CStyle, // comments starting with `/*` and ending with `*/`.
    DoubleSlash, // comments starting with `//` which are line terminal.
    Octothorpe, // comments starting with `#` which are line terminal.
}

pub fn remove_comments(src: &str, comment_type: CommentType) -> String {
    lazy_regex! {
        C_STYLE_COMMENT = r"/\*[^\*]*\*/";
        DOUBLE_SLASH_COMMENT = r"//.*";
        OCTOTHORPE_COMMENT = r"#.*";
    }

    let remove = |re: &Regex| re.replace_all(src, "").to_string();
    
    match comment_type {
        CommentType::CStyle => remove(&C_STYLE_COMMENT),
        CommentType::DoubleSlash => remove(&DOUBLE_SLASH_COMMENT),
        CommentType::Octothorpe => remove(&OCTOTHORPE_COMMENT),
    }
}

pub fn log(msg: &str) {
    println!("{}: {}", "[ LOG ]".cyan().bold(), msg);
}

pub fn warning(msg: &str) {
    println!("{}: {}", "[ WARNING ]".yellow().bold(), msg);
}

pub fn error(msg: &str) -> ! {
    println!("{}: {}", "[ ERROR ]".red().bold(), msg);
    process::exit(-1);
}

#[derive(Logos, PartialEq)]
pub enum Token {
    // preprocessor tokens.
    #[token(".define_macro")]
    DefineMacro,

    #[token(".macro")]
    Macro,

    #[token(".include")]
    Include,

    #[token(".link")]
    Link,

    #[token(".format")]
    Format,

    // main language tokens.
    #[token("main")]
    Main,

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

    #[token("table")]
    Table,

    #[token("row")]
    Row,

    // other / generally applicable tokens.
    #[token("{")]
    BlockStart,

    #[token("}")]
    BlockEnd,

    #[error]
    Error,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let text = match self {
            Token::DefineMacro => "macro definition",
            Token::Macro => "macro substitution",
            Token::Include => "file inclusion",
            Token::Link => "link",
            Token::Main => "main",
            Token::Chapter => "chapter",
            Token::Section => "section",
            Token::Subsection => "subsection",
            Token::Text => "text",
            Token::BlockStart => "block start",
            Token::BlockEnd => "block end",
            _ => "other",
        };

        write!(f, "{}", text)
    }
}

fn skip_block(lex: &mut Lexer<Token>) {
    while let Some(tok) = lex.next() {
        match tok {
            Token::BlockStart => skip_block(lex),
            Token::BlockEnd => break,
            _ => (),
        }
    }
}

pub fn expect_token(lex: &mut Lexer<Token>, expected: Token) {
    match lex.next() {
        None => error("expecting token when none available"),
        Some(tok) => if tok != expected {
            error(&format!("expected {}, found {}", expected, tok));
        }
    }
}

pub fn extract_arg(lex: &mut Lexer<Token>, src: &str) -> String {
    expect_token(lex, Token::BlockStart);
    
    let start = lex.span().end;
    skip_block(lex);
    let end = lex.span().start;

    src[start..end].to_string()
}
