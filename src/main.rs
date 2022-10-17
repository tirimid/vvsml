pub mod preprocess;
pub mod lang_util;
pub mod ipa_trans;

fn main() {
    use std::fs;

    let src = fs::read_to_string("design/basic.vvsml").unwrap();
    let src = preprocess::preprocess(&src);
    println!("{}", src);
}
