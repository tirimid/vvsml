use logos::Logos;

use crate::lazy_regex;
use crate::lang_util::FindRev;

#[derive(Logos)]
enum VvtabToken {
    #[token("&")]
    NextItem,
    
    #[token("$")]
    NextRow,
    
    #[error]
    Error,
}

pub fn vvtab_to_vvsml(src: &str) -> String {
    lazy_regex! {
        WHITESPACE = r"\s+";
        ESCAPE_CHAR = r"\\[\&\$]";
        PROTECTED_SEQ = r"@#':\[;:[A-Z][A-Z0-9_]\]";
    }

    // protect escape characters.
    // note that `\` is used as the escape character, but only for `&` and `$`.
    // i.e. `\$` or `\&` will be protected, but `\^` will not.
    let mut src = src.to_string();
    for mat in ESCAPE_CHAR.find_rev(&src.clone()) {
        let replacement = match src.chars().nth(mat.start() + 1).unwrap() {
            '&' => "@#':[;:AS]",
            '$' => "@#':[;:DS]",
            _ => continue,
        };

        src.replace_range(mat.range(), &replacement);
    }

    // extract text table information.
    let mut lex = VvtabToken::lexer(&src);
    let mut all_rows = Vec::new();
    let mut cur_row = Vec::new();
    let mut accum = String::new();

    while let Some(tok) = lex.next() {
        match tok {
            VvtabToken::NextItem => {
                cur_row.push(accum);
                accum = String::new();
            }
            VvtabToken::NextRow => {
                // a trailing `&` is not necessary for the last item in a row.
                if accum.len() > 0 {
                    cur_row.push(accum);
                    accum = String::new();
                }

                all_rows.push(cur_row);
                cur_row = Vec::new();
            }
            _ => accum += lex.slice(),
        }
    }

    // a trailing `$` should not be necessary for the last row, nor should a
    // trailing `&` be necessary for the last item.
    if accum.len() > 0 {
        cur_row.push(accum);
    }

    if cur_row.len() > 0 {
        all_rows.push(cur_row);
    }

    // convert extracted information to vvsml.
    let mut out = String::from("table{");
    for row in all_rows {
        out += "row{";
        for item in row {
            out += &format!("text{{{}}}", item);
        }

        out += "}";
    }
    
    out += "}";

    // deprotect protected sequences.
    // note that invalid vvtab sequences are simply removed.
    for mat in PROTECTED_SEQ.find_rev(&out.clone()) {
        let prot_code = out[(mat.start() + 7)..(mat.start() + 9)].to_string();
        let replacement = match prot_code.as_ref() {
            "AS" => "&",
            "DS" => "$",
            _ => "",
        };

        out.replace_range(mat.range(), &replacement);
    }

    WHITESPACE.replace_all(&out, " ").to_string()
}
