#![allow(non_snake_case)]
#[cfg(test)]
extern crate uuid;

pub mod lexer;
pub mod parser;

fn main() {
    let _parsed = lexer::KBuff::new("def foo(x, y) x + y");
    let _parsed = parser::Parser::new(4);

    // dbg!(parsed);
}
