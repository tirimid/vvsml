pub mod preprocess;
pub mod lang_util;
pub mod ipa_trans;
pub mod parse;
pub mod code_gen;
pub mod postprocess;

fn main() {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::env;

    let args = env::args()
        .into_iter()
        .map(|arg| arg.to_string())
        .collect::<Vec<_>>();
    
    if args.len() != 3 {
        lang_util::error("incorrect usage! correct: `vvsml <source> <output>`");
    }

    let html = {
        let src = fs::read_to_string(&args[1])
            .unwrap_or_else(|_| lang_util::error("invalid source file!"));
        
        let src = preprocess::preprocess(&src);
        let ast = parse::parse(&src);
        let html = code_gen::ast_to_html(&ast);
        let html = postprocess::postprocess(&html);

        html
    };

    File::create(&args[2])
        .unwrap_or_else(|_| lang_util::error("invalid output file!"))
        .write_all(&html.bytes().collect::<Vec<_>>())
        .unwrap_or_else(|_| lang_util::error("cannot write output file!"));
}
