pub mod ast;
use super::lexer::*;
pub struct ASTNode {}

pub struct Parser {
    parsed_tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(lexer: &mut KBuff) -> Self {
        let parsed_tokens = vec![0; 5].into_iter().map(|_| lexer.next_token()).collect();
        Parser {
            parsed_tokens,
            pos: 0,
        }
    }

    fn consume(&mut self, lexer: &mut KBuff) {
        self.parsed_tokens[self.pos] = lexer.next_token();
    }
    fn sync(&mut self, i: usize, lexer: &mut KBuff) {
        if self.pos + i > self.parsed_tokens.len() {
            self.fill((self.pos + i - 1) - (self.parsed_tokens.len() - 1), lexer);
        }
    }

    fn fill(&mut self, i: usize, lexer: &mut KBuff) {
        self.parsed_tokens
            .extend(vec![0; i].iter().map(|_| lexer.next_token()));
    }

    fn LT(&mut self, i: usize, lexer: &mut KBuff) -> Token {
        self.sync(i, lexer);
        self.parsed_tokens[self.pos + i - 1].clone()
    }

    pub fn LA(&mut self, i: usize, lexer: &mut KBuff) -> Token {
        self.LT(i, lexer)
    }

    pub fn r#match(&mut self, tok: &Token, lexer: &mut KBuff) {
        if self.LA(1, lexer) == *tok {
            self.consume(lexer);
        }
    }

    pub fn parse(&mut self, lexer: &mut KBuff, ast: &Vec<ASTNode>) -> Result<(), String> {
        loop {
            match self.LA(1, lexer) {
                Token::Extern => {
                    self.parse_extern(lexer, ast)?;
                }
                Token::Def => {
                    self.parse_def(lexer, ast)?;
                }
                _ => break,
            }
        }
        Ok(())
    }

    pub fn parse_def(&mut self, lexer: &mut KBuff, ast: &Vec<ASTNode>) -> Result<(), String> {
        self.consume(lexer);
        self.prototype(lexer, ast)
    }

    pub fn parse_extern(&mut self, lexer: &mut KBuff, ast: &Vec<ASTNode>) -> Result<(), String> {
        self.consume(lexer);
        self.prototype(lexer, ast)
    }

    pub fn prototype(&mut self, lexer: &mut KBuff, _ast: &Vec<ASTNode>) -> Result<(), String> {
        let _name = match self.LA(1, lexer) {
            Token::Ident(name) => {
                self.consume(lexer);
                name
            }
            other => return Err(format!("Found {:?}", other)),
        };

        match self.LA(1, lexer) {
            Token::LParenthesis => self.consume(lexer),
            other => return Err(format!("Found {:?}", other)),
        }

        let mut args: Vec<String> = Vec::new();
        while let Token::Ident(arg) = self.LA(args.len() + 1, lexer) {
            self.consume(lexer);
            args.push(arg);
            match self.LA(1, lexer) {
                Token::Comma => {
                    self.consume(lexer);
                    continue;
                }
                _ => break,
            }
        }

        match self.LA(args.len(), lexer) {
            Token::RParenthesis => self.consume(lexer),
            other => return Err(format!("Found {:?}", other)),
        }
        Ok(())
    }
}
