mod parser;
use parser::parse;
extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() {
    println!("Hello, world!");

    let src_code = std::fs::read_to_string("test.txt")
            .expect("Could not read source file");
    parse(src_code);
}
