pub mod ast;
use super::lexer::{KBuff, Token, Token::*};
use ast::{ProtoType, AST, AST::*};

use std::cell::RefCell;

pub struct Parser<'a> {
    k: usize,
    pos: usize,
    look_ahead: Vec<Token>,
    lexer: RefCell<KBuff<'a>>,
}

macro_rules! expect_token {
    ($token:tt, $func_name:ident, $parser:ident, $err:expr) => {
        match $parser.next_token(1) {
            $token($func_name) => {
                $parser.consume();
                $func_name
            }
            _ => panic!($err),
        };
    };
    ($token:tt, $parser:ident, $err:expr) => {
        match $parser.next_token(1) {
            $token => $parser.consume(),
            _ => panic!($err),
        };
    };
}

impl<'a> Parser<'a> {
    pub fn new(k: usize, lexer: KBuff<'a>) -> Self {
        Parser {
            k,
            pos: 0,
            look_ahead: Vec::new(),
            lexer: RefCell::new(lexer),
        }
    }

    pub fn fill_look_ahead(&mut self) {
        self.look_ahead.append(
            &mut vec![0; self.k]
                .into_iter()
                .map(|_| self.lexer.borrow_mut().next_token())
                .collect::<Vec<Token>>(),
        );
    }

    fn next_token(&mut self, i: usize) -> Token {
        self.look_ahead[(self.pos + i - 1) % self.k].clone()
    }

    fn consume(&mut self) {
        self.look_ahead[self.pos] = self.lexer.borrow_mut().next_token();
        self.pos = (self.pos + 1) % self.k;
    }
}

pub fn parse(parser: &mut Parser) -> Vec<AST> {
    parser.fill_look_ahead();
    let mut ast = Vec::new();
    loop {
        match (parser.next_token(1), parser.next_token(2)) {
            _ => {}
        }

        match parser.next_token(1) {
            Extern => ast.push(parse_extern(parser)),
            _ => break,
        }
    }
    ast
}

fn parse_extern(parser: &mut Parser) -> AST {
    parser.consume();

    let name = expect_token!(Ident, func_name, parser, "Expected function name");
    expect_token!(LParenthesis, parser, "Expected Left paren");

    let mut args = Vec::new();
    args.push(expect_token!(Ident, arg, parser, "Expected Arg"));

    while let Comma = parser.next_token(1) {
        parser.consume();

        args.push(expect_token!(Ident, arg, parser, "Expected Arg"));
    }

    parser.consume();
    expect_token!(RParenthesis, parser, "Expected Right paren");

    ExternNode(ProtoType::new(name, args))
}

#[cfg(test)]
mod test {
    // use super::AST::*;
    // use super::*;

    #[test]
    fn test_list() {
        /*         let mut parser = Parser::new(4); */
        // let mut lexer = KBuff::new("[ Kevin, other , other]");
        // assert_eq!(
        //     vec![List(
        //         Token::LBracket,
        //         vec![
        //             Element("Kevin".to_owned()),
        //             Element("other".to_owned()),
        //             Element("other".to_owned())
        //         ],
        //         Token::RBracket
        //     )],
        //     parser.parse(&mut lexer)
        // );
        // let mut parser = Parser::new(4);
        // let mut lexer = KBuff::new("[ Kevin, other,  [ Kevin ], other]");
        // assert_eq!(
        //     vec![List(
        //         Token::LBracket,
        //         vec![
        //             Element("Kevin".to_owned()),
        //             Element("other".to_owned()),
        //             List(
        //                 Token::LBracket,
        //                 vec![Element("Kevin".to_owned())],
        //                 Token::RBracket
        //             ),
        //             Element("other".to_owned())
        //         ],
        //         Token::RBracket
        //     )],
        //     parser.parse(&mut lexer)
        // );
        // let mut parser = Parser::new(4);
        // let mut lexer = KBuff::new("[ Kevin, other,  [ Kevin ], [other, [] ] ]");
        // assert_eq!(
        //     vec![List(
        //         Token::LBracket,
        //         vec![
        //             Element("Kevin".to_owned()),
        //             Element("other".to_owned()),
        //             List(
        //                 Token::LBracket,
        //                 vec![Element("Kevin".to_owned())],
        //                 Token::RBracket
        //             ),
        //             List(
        //                 Token::LBracket,
        //                 vec![
        //                     Element("other".to_owned()),
        //                     List(Token::LBracket, vec![], Token::RBracket)
        //                 ],
        //                 Token::RBracket
        //             ),
        //         ],
        //         Token::RBracket
        //     )],
        //     parser.parse(&mut lexer)
        // );
        // let mut parser = Parser::new(4);
        // let mut lexer = KBuff::new("[]");
        // assert_eq!(
        //     vec![List(Token::LBracket, vec![], Token::RBracket)],
        //     parser.parse(&mut lexer)
        // );

        // let mut parser = Parser::new(4);
        // let mut lexer = KBuff::new("Kevin = Kevin");
        // assert_eq!(
        //     vec![Assignment(
        //         "Kevin".to_owned(),
        //         Token::Operator("=".to_owned()),
        //         "Kevin".to_owned()
        //     )],
        //     parser.parse(&mut lexer)
        /* ); */
    }
}
