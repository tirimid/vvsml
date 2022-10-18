use std::collections::HashMap;
use std::fs;

use logos::Logos;

use crate::lazy_regex;
use crate::lang_util;
use crate::ipa_trans;
use crate::lang_util::{FindRev, Token};
use crate::ipa_trans::IpaTranslate;

// "protected" escape characters are deprotected during postprocessing.
// vvsml uses the following protection codes:
// `\{` => `@#':[;:LB]`
// `\}` => `@#':[;:RB]`
// `\\` => `@#':[;:BS]`
// `\.` => `@#':[;:P_]`
fn protect_escape(src: &str) -> String {
    lazy_regex! {
        BACKSLASH = r"\\[\s\S]?";
    }

    let mut new_src = src.to_string();
    for mat in BACKSLASH.find_rev(src) {
        let escape_ch = match src.chars().nth(mat.start() + 1) {
            Some(ch) => ch,
            None => lang_util::error("escaping inescapable character"),
        };

        let replacement = match escape_ch {
            '{' => "@#':[;:LB]",
            '}' => "@#':[;:RB]",
            '\\' => "@#':[;:BS]",
            '.' => "@#':[;:P_]",
            _ => lang_util::error(&format!("{} cannot be escaped!", escape_ch)),
        };

        new_src.replace_range(mat.range(), replacement);
    }

    new_src
}

fn verify_brace_balance(src: &str) {
    lazy_regex! {
        LEFT_BRACE = r"\{";
        RIGHT_BRACE = r"\}";
    }

    let left_braces = LEFT_BRACE.find_iter(src).count();
    let right_braces = RIGHT_BRACE.find_iter(src).count();

    if left_braces != right_braces {
        lang_util::error(
            &format!(
                "unbalanced braces! {} {{'s and {} }}'s found",
                left_braces,
                right_braces,
            ),
        );
    }
}

fn fulfill_macros(src: &str) -> String {
    let mut src = src.to_string();
    
    // extract macro definitions.
    let mut macro_defs = HashMap::new();
    let mut lex = Token::lexer(&src);
    while let Some(tok) = lex.next() {
        if tok != Token::DefineMacro {
            continue;
        }
        
        let def_start = lex.span().start;
        let name = lang_util::extract_arg(&mut lex, &src);
        let conts = lang_util::extract_arg(&mut lex, &src);
        let def_end = lex.span().end;

        macro_defs.insert(name, conts);

        src.replace_range(def_start..def_end, "");
        lex = Token::lexer(&src);
    }

    // fulfill macro substitutions.
    let mut lex = Token::lexer(&src);
    while let Some(tok) = lex.next() {
        if tok != Token::Macro {
            continue;
        }

        let macro_start = lex.span().start;
        let name = lang_util::extract_arg(&mut lex, &src);
        let macro_end = lex.span().end;

        let conts = match macro_defs.get(&name) {
            Some(conts) => conts,
            None => lang_util::error(&format!("macro not defined: {}", &name)),
        };

        src.replace_range(macro_start..macro_end, &conts);
        lex = Token::lexer(&src);
    }
    
    lang_util::log("fulfilled macros");
    src
}

fn fulfill_includes(src: &str) -> String {
    let mut src = src.to_string();

    let mut lex = Token::lexer(&src);
    while let Some(tok) = lex.next() {
        if tok != Token::Include {
            continue;
        }

        let include_start = lex.span().start;
        let path = lang_util::extract_arg(&mut lex, &src);
        let include_end = lex.span().end;

        let file_conts = match fs::read_to_string(&path) {
            Ok(conts) => conts,
            Err(_) => lang_util::error(&format!("cannot include {}", &path)),
        };

        src.replace_range(include_start..include_end, &file_conts);
        lex = Token::lexer(&src);
    }

    src
}

fn fulfill_links(src: &str) -> String {
    let mut src = src.to_string();

    let mut lex = Token::lexer(&src);
    while let Some(tok) = lex.next() {
        if tok != Token::Link {
            continue;
        }
        
        let link_start = lex.span().start;
        let name = lang_util::extract_arg(&mut lex, &src);
        let dst = lang_util::extract_arg(&mut lex, &src);
        let link_end = lex.span().end;

        src.replace_range(
            link_start..link_end,
            &format!("<a href=\"{}\">{}</a>", &dst, &name),
        );
        
        lex = Token::lexer(&src);
    }

    lang_util::log("fulfilled links");
    src
}

fn single_format(spec: &str, text: &str) -> String {
    let mut text = text.to_string();
    let mut spec_cnt = HashMap::new();

    let tag_surround = |open, close, text: &mut String| {
        *text = format!("{}{}{}", open, text, close);
    };

    for ch in spec.chars() {
        match ch {
            'b' => tag_surround("<b>", "</b>", &mut text),
            'i' => tag_surround("<i>", "</i>", &mut text),
            '_' => tag_surround("<sub>", "</sub>", &mut text),
            '^' => tag_surround("<sup>", "</sup>", &mut text),
            'x' => text = ipa_trans::XSAMPA_TRANS.translate(&text),
            _ => lang_util::error(&format!("invalid format specifier: {}", ch)),
        }

        if spec_cnt.insert(ch, true) != None {
            let msg = format!("format specifier {} is redundant", ch);
            lang_util::warning(&msg);
        }
    }

    text
}

fn fulfill_formats(src: &str) -> String {
    let mut src = src.to_string();

    let mut lex = Token::lexer(&src);
    while let Some(tok) = lex.next() {
        if tok != Token::Format {
            continue;
        }
        
        let fmt_start = lex.span().start;
        let spec = lang_util::extract_arg(&mut lex, &src);
        let text = lang_util::extract_arg(&mut lex, &src);
        let fmt_end = lex.span().end;

        src.replace_range(fmt_start..fmt_end, &single_format(&spec, &text));
        lex = Token::lexer(&src);
    }

    lang_util::log("fulfilled formats");
    src.to_string()
}

fn work_left_to_do(src: &str) -> bool {
    let mut lex = Token::lexer(src);
    let mut directive_cnt = 0;
    while let Some(tok) = lex.next() {
        match tok {
            Token::DefineMacro |
            Token::Macro |
            Token::Include |
            Token::Link |
            Token::Format => directive_cnt += 1,
            _ => (),
        }
    }

    directive_cnt > 0
}

pub fn preprocess(src: &str) -> String {
    // all consecutive stages of preprocessing, in order.
    // the completion of these stages is logged.
    let pipeline = [
        fulfill_macros,
        fulfill_includes,
        fulfill_links,
        fulfill_formats,
    ];

    let mut src = protect_escape(src);
    verify_brace_balance(&src);

    let mut pass = 1;
    while work_left_to_do(&src) {
        lang_util::log(&format!("pass: {}", pass));
        
        for stage in pipeline {
            src = stage(&src);
            src = protect_escape(&src);
            verify_brace_balance(&src);
        }

        pass += 1;
    }

    lang_util::log("preprocessing complete");
    lang_util::collapse_whitespace(&src)
}
