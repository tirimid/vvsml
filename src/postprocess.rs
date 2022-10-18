use crate::lazy_regex;
use crate::lang_util;
use crate::lang_util::FindRev;

fn deprotect_escape(html: &str) -> String {
    lazy_regex! {
        PROTECTED_CHAR = r"@#':\[;:[A-Z][A-Z_]\]";
    }

    let mut html = html.to_string();
    for mat in PROTECTED_CHAR.find_rev(&html.clone()) {
        let prot = html[mat.range()].to_string();
        let ch = match prot.as_str() {
            "@#':[;:LB]" => '{',
            "@#':[;:RB]" => '}',
            "@#':[;:BS]" => '\\',
            "@#':[;:P_]" => '.',
            _ => lang_util::error(&format!("bad protected sequence: {}", prot)),
        };

        html.replace_range(mat.range(), &ch.to_string());
    }

    lang_util::log("deprotected escape sequences");
    html.to_string()
}

pub fn postprocess(html: &str) -> String {
    let html = deprotect_escape(html);

    lang_util::log("postprocessing complete");
    html
}
