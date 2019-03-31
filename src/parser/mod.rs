pub mod ast;
use super::lexer::*;

#[derive(Debug)]
pub enum AST {
    List(Token, Vec<AST>, Token),
    Element(String),
}

struct Parser {}

impl<'a> Parser {
    pub fn new() -> Self {
        Parser {}
    }

    fn parse(&self, lexer: &mut KBuff) -> Vec<AST> {
        let mut ast = Vec::new();
        loop {
            match lexer.next_token() {
                Token::Ident(name) => ast.push(AST::Element(name)),
                Token::LBracket => ast.push(self.list(lexer)),
                Token::EOF => break,
                _ => {}
            }
        }
        ast
    }

    fn list(&self, lexer: &mut KBuff) -> AST {
        let mut ast = Vec::new();
        match self.elements(lexer, &mut ast) {
            Token::RBracket => {}

            _ => panic!("Expected closing"),
        }
        AST::List(Token::RBracket, ast, Token::LBracket)
    }

    fn elements(&self, lexer: &mut KBuff, ast: &mut Vec<AST>) -> Token {
        match lexer.next_token() {
            Token::Ident(name) => ast.push(AST::Element(name.to_owned())),
            x => panic!("Expected Name found {:?}", x),
        }
        while let Token::Comma = lexer.next_token() {
            match lexer.next_token() {
                Token::Ident(name) => ast.push(AST::Element(name)),
                Token::LBracket => ast.push(self.list(lexer)),
                Token::RBracket => return Token::RBracket,
                _ => panic!("Expect Comma"),
            };
        }
        Token::RBracket
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::*;

    #[test]
    fn test_list() {
        let parser = Parser::new();
        let mut lexer = KBuff::new("[ Kevin, Bryan, [ Kevin ] , other]");
        dbg!(parser.parse(&mut lexer));
    }
}
