use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FartParser;

pub enum Node {
    Definition(String),

}

pub fn parse(code: String) {

}