#[macro_use]
pub mod lang_util;

pub mod ipa_trans;
pub mod preproc;

fn main() {
    let file_path = "design/basic.vvsml";
    let src = std::fs::read_to_string(file_path).unwrap();
    let src = preproc::preprocess(file_path, &src);
    println!("{}", src);
}
