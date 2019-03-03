#![allow(non_snake_case)]
use std::env;

pub mod parser;

fn main() {
    let parsed = parser::parse(env::args().last().unwrap());
    dbg!(parsed);
}
