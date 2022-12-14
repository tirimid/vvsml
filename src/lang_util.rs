use std::ops::Range;
use std::fmt::Display;
use std::process;

use regex::{Regex, Match};
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

pub trait CountLines {
    fn count_lines(&self) -> usize;
    fn count_lines_in(&self, range: Range<usize>) -> usize;
}

impl CountLines for &str {
    fn count_lines(&self) -> usize {
        self.chars().filter(|c| *c == '\n').count()
    }

    fn count_lines_in(&self, range: Range<usize>) -> usize {
        self
            .chars()
            .enumerate()
            .filter(|(i, _)| range.contains(&i))
            .filter(|(_, c)| *c == '\n')
            .count()
    }
}

impl CountLines for String {
    fn count_lines(&self) -> usize {
        (self as &str).count_lines()
    }

    fn count_lines_in(&self, range: Range<usize>) -> usize {
        (self as &str).count_lines_in(range)
    }
}

#[macro_export]
macro_rules! define_logger {
    ($name:ident, $tag:expr) => {
        #[macro_export]
        macro_rules! $name {
            ($$file:expr, $$line:expr, $$msg:expr $$(,)*) => {
                {
                    use colored::Colorize;
                    println!("{} {}:{} - {}", $tag, $$file, $$line, $$msg);
                }
            };
            
            ($$file:expr, $$msg:expr $$(,)*) => {
                {
                    use colored::Colorize;
                    println!("{} {} - {}", $tag, $$file, $$msg);
                }
            };
            
            ($$msg:expr $$(,)*) => {
                {
                    use colored::Colorize;
                    println!("{} {}", $tag, $$msg);
                }
            };
        }
    };
}

define_logger!(log, "log".blue().bold());
define_logger!(warning, "warning".yellow().bold());
define_logger!(error, "error".red().bold());

#[macro_export]
macro_rules! lazy_regex {
    ($($n:ident = $r:literal;)*) => {
        lazy_static::lazy_static! {
            $(static ref $n: regex::Regex = regex::Regex::new($r).unwrap();)*
        }
    };
}

pub fn current_line<'a, T: Logos<'a>>(src: &str, lex: &Lexer<'a, T>) -> usize {
    1 + src.count_lines_in(0..lex.span().start)
}

pub fn expect_tok<'a, T: Logos<'a> + Display + PartialEq>(
    file_path: &str,
    src: &str,
    lex: &mut Lexer<'a, T>,
    exp: T,
) {
    match lex.next() {
        Some(tok) => if tok != exp {
            let err_msg = format!("expected token {} but found {}", exp, tok);
            error!(file_path, current_line(src, lex), err_msg);
            process::exit(-1);
        }
        None => {
            let err_msg = format!("expected token {} but found nothing", exp);
            error!(file_path, current_line(src, lex), err_msg);
            process::exit(-1);
        }
    }
}

fn skip_block<'a, T: Logos<'a> + PartialEq + Copy>(
    lex: &mut Lexer<'a, T>,
    block_start: T,
    block_end: T,
) {
    while let Some(tok) = lex.next() {
        if tok == block_start {
            skip_block(lex, block_start, block_end);
        } else if tok == block_end {
            break;
        }
    }
}

pub fn extract_arg<'a, T: Logos<'a> + Display + PartialEq + Copy>(
    file_path: &str,
    src: &str,
    lex: &mut Lexer<'a, T>,
    block_start: T,
    block_end: T,
) -> String {
    expect_tok(file_path, src, lex, block_start);
    let arg_start = lex.span().end;
    skip_block(lex, block_start, block_end);
    let arg_end = lex.span().start;
    src[arg_start..arg_end].to_string()
}
