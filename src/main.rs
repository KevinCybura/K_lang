#![allow(non_snake_case)]
#[cfg(test)]
extern crate uuid;

use std::env;

pub mod ast;
pub mod parser;

fn main() {
    let parsed = parser::parse(env::args().last().unwrap());
    dbg!(parsed);
}
