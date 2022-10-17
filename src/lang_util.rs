use std::process;

use regex::{Regex, Match};
use colored::Colorize;

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
