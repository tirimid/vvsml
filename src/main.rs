#[macro_use]
pub mod lang_util;

pub mod ipa_trans;
pub mod preproc;
pub mod parse;

fn main() {
    let file_path = "design/basic.vvsml";
    let src = std::fs::read_to_string(file_path).unwrap();
    let src = preproc::preprocess(file_path, &src);
    let ast = parse::parse(file_path, &src);
    println!("{:#?}", ast);
}
