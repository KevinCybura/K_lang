pub mod ast;
use super::lexer::*;

struct Parser<'a> {
    lexer: KBuff<'a>,
    look_ahead: Vec<Token>,
    parsed_tokens: Vec<Token>,
    k: usize,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, k: usize) -> Self {
        let mut lexer = KBuff::new(input);
        let look_ahead = vec![0; k].into_iter().map(|_| lexer.next_token()).collect();
        Parser {
            lexer,
            look_ahead,
            parsed_tokens: Vec::new(),
            k,
            pos: 0,
        }
    }

    fn consume(&mut self) {
        self.look_ahead[self.pos] = self.lexer.next_token();
    }

    pub fn LT(&self, i: usize) -> &Token {
        &self.look_ahead[(self.pos + i - 1) % self.k]
    }

    pub fn LA(&self, i: usize) -> &Token {
        self.LT(i)
    }

    pub fn r#match(&mut self, tok: &Token) {
        if self.LA(1) == tok {
            self.consume();
        }
    }

    pub fn prototype(&mut self) {
        if let Token::Ident(name) = self.LA(1) {
            self.r#match(&Token::Ident(name.to_owned()));
        }

        if let Token::LParenthesis = self.LA(1) {
            self.r#match(&Token::LParenthesis);
        }

        loop {

        }

    }
}
