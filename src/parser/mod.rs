pub mod ast;
use super::lexer::{KBuff, Token, TokenType, TokenType::*};
use ast::{Expression, Expression::*, ProtoType, AST, AST::*};

use std::cell::RefCell;

pub struct Parser<'a> {
    k: usize,
    pos: usize,
    look_ahead: Vec<Token>,
    lexer: RefCell<KBuff<'a>>,
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

    // TODO:
    // fn extend_look_ahead(&mut self) {
    //     vec![0; self.look_ahead.len()] self.lexer
    //         .borrow_mut()
    //         .next_token()
    //         .take(self.look_ahead.len());
    // }

    fn next_token(&self, i: usize) -> &TokenType {
        // TODO:
        // if i >= self.look_ahead.len() {
        //     self.extend_look_ahead();
        // }
        &self.look_ahead[(self.pos + i - 1) % self.k].token_t
    }

    fn token(&mut self, i: usize) -> Token {
        // if i >= self.look_ahead.len() {
        //     self.extend_look_ahead();
        // }
        let res = self.look_ahead[(self.pos + i - 1) % self.k].clone();
        self.consume();
        res
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

// fn parse_def(parser: &mut Parser) -> AST {
//     parser.consume();
//     let prototype = parse_prototype(parser);

//     // let expression = parse_expr(parser);

//     ExternNode(prototype)
// }

pub fn parse_expr(parser: &mut Parser) -> AST {
    match parser.next_token(1) {
        Ident => Expr(parse_binary_expr(parser)),
        Operator => Expr(parse_unary_expr(parser)),
        _ => panic!("Expected Ident or Operator found {:?}", parser.token(1)),
    }
}

fn parse_unary_expr(parser: &mut Parser) -> Expression {
    match (parser.next_token(1), parser.next_token(2)) {
        (Ident, _) => return parse_primary(parser),
        (Operator, Ident) => {
            let op = parser.token(1);
            UnaryExpr(op.lexeme, Box::new(parse_primary(parser)))
        }
        _ => panic!(
            "Expect Ident , _ or Operator, Ident found : {:?} and {:?}",
            parser.token(1),
            parser.token(1)
        ),
    }
}

fn parse_primary(parser: &mut Parser) -> Expression {
    match parser.next_token(1) {
        Numeric | String => LiteralEpxr(parser.token(1)),
        Ident => VariableExpr(parser.token(1)),
        _ => panic!("Expected variable or literal found {:?}", parser.token(1)),
    }
}

fn parse_binary_expr(parser: &mut Parser) -> Expression {
    let lhs = match parser.next_token(1) {
        Ident | Operator => parse_unary_expr(parser),
        _ => panic!("Expected expresion found {:?}", parser.token(1)),
    };

    let op = match parser.next_token(1) {
        Operator => parser.token(1),
        _ => return lhs,
    };

    let rhs = match parser.next_token(1) {
        Ident => parse_unary_expr(parser),
        _ => panic!("Expected Ident found {:?}", parser.token(1)),
    };

    BinaryExpr(op.lexeme, Box::new(lhs), Box::new(rhs))
}

fn parse_extern(parser: &mut Parser) -> AST {
    parser.consume();
    let proto = parse_prototype(parser);

    ExternNode(proto)
}

fn parse_prototype(parser: &mut Parser) -> ProtoType {
    let name = match parser.next_token(1) {
        Ident => parser.token(1),
        _ => panic!("Expected function name found : {:?}", parser.token(1)),
    };

    match parser.next_token(1) {
        LParenthesis => parser.consume(),
        _ => panic!("Expected Open LParenthesis found : {:?}", parser.token(1)),
    }

    let mut args = Vec::new();
    loop {
        match parser.next_token(1) {
            Ident => args.push(parser.token(1)),
            Comma => parser.consume(),
            RParenthesis => break,
            _ => panic!(
                "Found {:?} when parsing args for {:?}",
                parser.token(1),
                name
            ),
        }
    }

    ProtoType::new(name, args)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_prototype_with_args() {
        // Prototype  : Ident OpeningParenthesis [Ident Comma ?]* ClosingParenthesis;
        let lexer = KBuff::new("foo(x, y)");
        let mut parser = Parser::new(4, lexer);
        parser.fill_look_ahead();
        let x = ProtoType::new(
            Token::new(Ident, "foo".to_owned(), 0),
            vec![
                Token::new(Ident, "x".to_owned(), 0),
                Token::new(Ident, "y".to_owned(), 0),
            ],
        );

        assert_eq!(x, parse_prototype(&mut parser));
    }

    #[test]
    fn test_parse_prototype_no_args() {
        let lexer = KBuff::new("foo()");
        let mut parser = Parser::new(4, lexer);
        parser.fill_look_ahead();
        let x = ProtoType::new(Token::new(Ident, "foo".to_owned(), 0), vec![]);

        assert_eq!(x, parse_prototype(&mut parser));
    }

    #[test]
    fn test_parse_extern() {
        //declaration : Extern prototype;
        let lexer = KBuff::new("extern foo(x, y)");
        let mut parser = Parser::new(4, lexer);
        parser.fill_look_ahead();
        let x = ExternNode(ProtoType::new(
            Token::new(Ident, "foo".to_owned(), 0),
            vec![
                Token::new(Ident, "x".to_owned(), 0),
                Token::new(Ident, "y".to_owned(), 0),
            ],
        ));

        assert_eq!(x, parse_extern(&mut parser));
    }
}
