use std::collections::HashMap;
use std::process;
use std::fmt;
use std::fmt::{Formatter, Display};

use logos::{Logos, Lexer};

use crate::lang_util;
use crate::ipa_trans;
use crate::lazy_regex;
use crate::lang_util::{FindRev, CountLines};
use crate::ipa_trans::IpaTranslate;

#[derive(Logos, PartialEq, Clone, Copy)]
enum Token {
    #[token(".define_macro")]
    DefineMacro,

    #[token(".macro")]
    Macro,

    #[token(".format")]
    Format,

    #[token(".link")]
    Link,

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
            Self::DefineMacro => "macro definition",
            Self::Macro => "macro substitution",
            Self::Format => "formatting statement",
            Self::Link => "link",
            Self::BlockStart => "block start",
            Self::BlockEnd => "block end",
            _ => "other",
        };

        write!(f, "{}", text)
    }
}

fn protect_seqs(file_path: &str, src: &str) -> String {
    lazy_regex! {
        // `]]$` is a very rare sequence of characters.
        ESCAPE_CHAR = r"]]\$[\s\S]?";
    }

    // protect escape characters.
    let mut src = src.to_string();
    for mat in ESCAPE_CHAR.find_rev(&src.clone()) {
        let line = 1 + src.count_lines_in(0..mat.start());
        let escape_ch = match src.chars().nth(mat.start() + 3) {
            Some(ch) => ch,
            None => {
                error!(file_path, line, "escaping inescapable character");
                process::exit(-1);
            }
        };

        let replacement = match escape_ch {
            '{' => "@#':[;:LB]",
            '}' => "@#':[;:RB]",
            ']' => "@#':[;:E1]",
            '$' => "@#':[;:E2]",
            '.' => "@#':[;:P_]",
            _ => {
                let err_msg = format!("{} cannot be escaped", escape_ch);
                error!(file_path, line, err_msg);
                process::exit(-1);
            }
        };

        src.replace_range(mat.range(), replacement);
    }

    src
}

fn extract_arg(file_path: &str, src: &str, lex: &mut Lexer<Token>) -> String {
    lang_util::expect_tok(file_path, src, lex, Token::BlockStart);
    let arg_start = lex.span().end;
    while let Some(tok) = lex.next() {
        match tok {
            Token::BlockStart => {
                let err_msg = "unescaped { in preprocessor argument";
                error!(file_path, lang_util::current_line(src, lex), err_msg);
                process::exit(-1);
            }
            Token::BlockEnd => break,
            _ => continue,
        }
    }

    let arg_end = lex.span().start;
    src[arg_start..arg_end].to_string()
}

fn define_macro(
    file_path: &str,
    src: &str,
    lex: &mut Lexer<Token>,
    sym_tab: &mut HashMap<String, String>,
) -> String {
    let def_start = lex.span().start;
    let name = extract_arg(file_path, src, lex);
    let conts = extract_arg(file_path, src, lex);
    let def_end = lex.span().end;

    let mut src = src.to_string();
    src.replace_range(def_start..def_end, "");
    sym_tab.insert(name, conts);
    src
}

fn r#macro(
    file_path: &str,
    src: &str,
    lex: &mut Lexer<Token>,
    sym_tab: &HashMap<String, String>,
) -> String {
    let macro_start = lex.span().start;
    let name = extract_arg(file_path, src, lex);
    let macro_end = lex.span().end;

    let conts = match sym_tab.get(&name) {
        Some(conts) => conts,
        None => {
            let err_msg = format!("macro not defined: {}", name);
            error!(file_path, lang_util::current_line(src, lex), err_msg);
            process::exit(-1);
        }
    };

    let mut src = src.to_string();
    src.replace_range(macro_start..macro_end, &conts);
    src
}

fn single_fmt(file_path: &str, line: usize, spec: &str, text: &str) -> String {
    let mut text = text.to_string();
    let mut spec_cnt = HashMap::new();

    let tag_surround = |open, close, text: &mut String| {
        *text = format!("{}{}{}", open, text, close);
    };

    for ch in spec.chars() {
        // a format specifier is only used once.
        // as to say, specification `bbbii_^^^^` is the same as `bi_^`.
        if spec_cnt.insert(ch, true) != None {
            let warn_msg = format!("format specifier {} is redundant", ch);
            warning!(file_path, line, warn_msg);
            continue;
        }
        
        match ch {
            'b' => tag_surround("<b>", "</b>", &mut text),
            'i' => tag_surround("<i>", "</i>", &mut text),
            '_' => tag_surround("<sub>", "</sub>", &mut text),
            '^' => tag_surround("<sup>", "</sup>", &mut text),
            'x' => text = ipa_trans::XSAMPA_TRANS.translate(&text),
            _ => {
                let err_msg = format!("invalid format specifier: {}", ch);
                error!(file_path, line, err_msg);
                process::exit(-1);
            }
        }
    }

    text
}

fn format(file_path: &str, src: &str, lex: &mut Lexer<Token>) -> String {
    let fmt_start = lex.span().start;
    let spec = extract_arg(file_path, src, lex);
    let text_line = lang_util::current_line(src, lex);
    let text = extract_arg(file_path, src, lex);
    let fmt_end = lex.span().end;

    let mut src = src.to_string();
    src.replace_range(
        fmt_start..fmt_end,
        &single_fmt(file_path, text_line, &spec, &text),
    );
    
    src
}

fn link(file_path: &str, src: &str, lex: &mut Lexer<Token>) -> String {
    let link_start = lex.span().start;
    let name = extract_arg(file_path, src, lex);
    let dst = extract_arg(file_path, src, lex);
    let link_end = lex.span().end;

    let mut src = src.to_string();
    src.replace_range(
        link_start..link_end,
        &format!("<a href=\"{}\">{}</a>", &dst, &name),
    );

    src
}

pub fn preprocess(file_path: &str, src: &str) -> String {
    let mut src = protect_seqs(file_path, src);
    let mut lex = Token::lexer(&src);
    let mut sym_tab = HashMap::new();
    while let Some(tok) = lex.next() {
        src = match tok {
            Token::DefineMacro => define_macro(
                file_path,
                &src,
                &mut lex,
                &mut sym_tab,
            ),
            Token::Macro => r#macro(file_path, &src, &mut lex, &sym_tab),
            Token::Format => format(file_path, &src, &mut lex),
            Token::Link => link(file_path, &src, &mut lex),
            _ => continue,
        };

        lex = Token::lexer(&src);
    }

    src
}
