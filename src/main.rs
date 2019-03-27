#![allow(non_snake_case)]
#[cfg(test)]
extern crate uuid;

pub mod lexer;
pub mod parser;

fn main() {
    // let parsed = parser::parse(env::args().last().unwrap());
    let parsed = lexer::KBuff::new("def foo(x, y) x + y").tokenize();
    dbg!(parsed);
}
