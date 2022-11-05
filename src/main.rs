#[macro_use]
pub mod lang_util;

pub mod preproc;
pub mod parse;
pub mod code_gen;

fn main() {
    use std::fs::File;
    use std::io::Write;
    use std::fs;
    use std::env;
    use std::process;
    
    if env::args().len() != 3 {
        error!("usage: `vvsml <source file> <output file>`");
        process::exit(-1);
    }

    let src_file = env::args().nth(1).unwrap();
    let dst_file = env::args().nth(2).unwrap();
    
    let src = fs::read_to_string(&src_file).unwrap();
    let src = preproc::preprocess(&src_file, &src);
    let ast = parse::parse(&src_file, &src);
    let html = code_gen::generate_html(&ast);

    File::create(&dst_file)
        .unwrap_or_else(|_| {
            error!("invalid output file");
            process::exit(-1);
        })
        .write_all(&html.bytes().collect::<Vec<_>>())
        .unwrap_or_else(|e| {
            error!(format!("unable to write output file: {}", e));
            process::exit(-1);
        });
}
