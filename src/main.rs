#![allow(non_snake_case)]

#[macro_use]
extern crate serde_derive;
extern crate docopt;

pub mod lexer;
pub mod parser;

use docopt::Docopt;

use std::io;
use std::io::prelude::*;

use parser::ast::*;

const USAGE: &'static str = "
Usage: iron_kale [(-l | -p | i)]

Options: 
    -l  Run only lexer and show its output.
    -p  Run only parser and show its output.
    -i  Run only IR builder and show its output.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_l: bool,
    flag_p: bool,
    flag_i: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stage {
    AST,
    Tokens,
}

pub fn main_loop(stage: Stage) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut parser_settings = default_parser_settings();

    'main: loop {
        print!("> ");
        stdout.flush().unwrap();
        input.clear();
        stdin
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        if input.as_str() == ".quit\n" {
            break;
        }

        let mut ast = Vec::new();

        let mut prev = Vec::new();
        loop {
            let tokens = lexer::tokenize(input.as_str());
            if stage == Stage::Tokens {
                println!("{:?}", tokens);
                continue 'main;
            }
            prev.extend(tokens.into_iter());

            let parsing_result = parse(prev.as_slice(), ast.as_slice(), &mut parser_settings);
            match parsing_result {
                Ok((parsed_ast, rest)) => {
                    ast.extend(parsed_ast.into_iter());
                    if rest.is_empty() {
                        break;
                    } else {
                        prev = rest;
                    }
                }
                Err(message) => {
                    println!("Error occured: {}", message);
                    continue 'main;
                }
            }
            print!(". ");
            stdout.flush().unwrap();
            input.clear();
            stdin
                .read_line(&mut input)
                .ok()
                .expect("Failed to read line");
        }
        if stage == Stage::AST {
            println!("{:?}", ast);
            continue;
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let stage = if args.flag_i {
        Stage::Tokens
    } else {
        Stage::AST
    };

    main_loop(stage);
    // let parsed = lexer::parse(env::args().last().unwrap());
    // dbg!(parsed);
}
