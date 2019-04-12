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

macro_rules! expect {
    ( [$($token:pat,  $result:stmt);+ ] <= $parser:ident,  $err:expr) => {
        match $parser.next_token(1) {
            $(
                $token => {
                $parser.consume();
                $result
            }
            )+
                _ => panic!($err),
        }
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
            // Def => ast.push(parse_def(parser)),
            _ => break,
        }
    }
    ast
}

fn parse_def(parser: &mut Parser) -> AST {
    parser.consume();
    let prototype = parse_prototype(parser);

    let expression = parse_expr(parser);

    ExternNode(prototype)
}

fn parse_expr(parser: &mut Parser) -> AST {

    let exper = expect!()
}

fn parse_extern(parser: &mut Parser) -> AST {
    parser.consume();
    let proto = parse_prototype(parser);

    ExternNode(proto)
}

fn parse_prototype(parser: &mut Parser) -> ProtoType {
    // let name = expect_token!(Ident, func_name, parser, "Expected function name");
    let name = expect!([Ident(name), name] <= parser, "Expected function name");
    expect!(
        [LParenthesis, LParenthesis] <= parser,
        "Expected Open parenthesis"
    );

    let mut args = Vec::new();
    loop {
        expect!(
        [Ident(arg), args.push(arg);
         Comma, continue ;
         RParenthesis, break] <= parser, "Expected Closing Paren"
        )
    }

    ProtoType::new(name, args)
}

#[cfg(test)]
mod test {
    use super::AST::*;
    use super::*;

    #[test]
    fn test_parse_prototype() {
        // Prototype  : Ident OpeningParenthesis [Ident Comma ?]* ClosingParenthesis;
        let lexer = KBuff::new("foo(x, y)");
        let mut parser = Parser::new(4, lexer);
        parser.fill_look_ahead();
        let x = ProtoType::new("foo".to_owned(), vec!["x".to_owned(), "y".to_owned()]);

        assert_eq!(x, parse_prototype(&mut parser));
    }

    #[test]
    fn test_parse_extern() {
        //declaration : Extern prototype;
        let lexer = KBuff::new("extern foo(x, y)");
        let mut parser = Parser::new(4, lexer);
        parser.fill_look_ahead();
        let x = ExternNode(ProtoType::new(
            "foo".to_owned(),
            vec!["x".to_owned(), "y".to_owned()],
        ));

        assert_eq!(x, parse_extern(&mut parser));
    }
}
