pub mod ast;
use super::lexer::*;

#[derive(Debug)]
pub enum AST {
    List(Token, Vec<AST>, Token),
    Element(String),
    Assignment(String, Token, String),
}

pub struct Parser {
    k: usize,
    pos: usize,
    look_ahead: Vec<Token>,
}

impl<'a> Parser {
    pub fn new(k: usize) -> Self {
        Parser {
            k,
            pos: 0,
            look_ahead: Vec::new(),
        }
    }

    pub fn fill_look_ahead(&mut self, lexer: &mut KBuff) {
        self.look_ahead.append(
            &mut vec![0; self.k]
                .into_iter()
                .map(|_| lexer.next_token())
                .collect::<Vec<Token>>(),
        );
    }

    fn next_token(&mut self, i: usize) -> Token {
        self.look_ahead[(self.pos + i - 1) % self.k].clone()
    }

    fn consume(&mut self, lexer: &mut KBuff) {
        self.look_ahead[self.pos] = lexer.next_token();
        self.pos = (self.pos + 1) % self.k;
    }

    pub fn parse(&mut self, lexer: &mut KBuff) -> Vec<AST> {
        self.fill_look_ahead(lexer);
        let mut ast = Vec::new();
        loop {
            match (self.next_token(1), self.next_token(2)) {
                (Token::Ident(ref name), Token::Operator(ref op)) if op == "=" => {
                    ast.push(self.assigment(lexer, name.to_owned()))
                }
                _ => {}
            }

            match self.next_token(1) {
                Token::LBracket => ast.push(self.list(lexer)),
                Token::Ident(name) => {
                    ast.push(self.name(lexer, name));
                }
                Token::EOF => break,
                _ => panic!(),
            }
        }
        ast
    }
    fn name(&mut self, lexer: &mut KBuff, name: String) -> AST {
        self.consume(lexer);
        AST::Element(name)
    }
    fn assigment(&mut self, lexer: &mut KBuff, ident: String) -> AST {
        self.consume(lexer);
        self.consume(lexer);
        match self.next_token(1) {
            Token::Ident(name) => {
                self.consume(lexer);
                AST::Assignment(ident, Token::Operator("=".to_owned()), name)
            }
            _ => panic!(),
        }
    }

    fn list(&mut self, lexer: &mut KBuff) -> AST {
        self.consume(lexer);
        let mut ast = Vec::new();
        self.elements(lexer, &mut ast);
        match self.next_token(1) {
            Token::RBracket => {}
            _ => panic!("Expected closing"),
        }
        self.consume(lexer);
        AST::List(Token::LBracket, ast, Token::RBracket)
    }

    fn elements(&mut self, lexer: &mut KBuff, ast: &mut Vec<AST>) {
        match self.next_token(1) {
            Token::Ident(name) => {
                self.consume(lexer);
                ast.push(AST::Element(name.to_owned()))
            }
            Token::RBracket => return,
            x => panic!("Expected Name found {:?}", x),
        }
        while let Token::Comma = self.next_token(1) {
            self.consume(lexer);
            match self.next_token(1) {
                Token::LBracket => ast.push(self.list(lexer)),
                Token::Ident(name) => ast.push(self.name(lexer, name)),
                Token::RBracket => return,
                x => panic!("Expect Comma: {:?}", x),
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list() {
        let mut parser = Parser::new(4);
        let mut lexer = KBuff::new("[ Kevin, other , other]");
        dbg!(parser.parse(&mut lexer));
        let mut parser = Parser::new(4);
        let mut lexer = KBuff::new("[ Kevin, other,  [ Kevin ], other]");
        dbg!(parser.parse(&mut lexer));
        let mut parser = Parser::new(4);
        let mut lexer = KBuff::new("[ Kevin, other,  [ Kevin ], [other, [] ] ]");
        dbg!(parser.parse(&mut lexer));
        let mut parser = Parser::new(4);
        let mut lexer = KBuff::new("[]");
        dbg!(parser.parse(&mut lexer));
        let mut parser = Parser::new(4);
        let mut lexer = KBuff::new("Kevin = Kevin");
        dbg!(parser.parse(&mut lexer));
    }
}
