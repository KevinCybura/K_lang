pub mod ast;
use super::lexer::{KBuff, Token, Token::*};
use ast::{ProtoType, AST, AST::*};

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
                _ => {}
            }

            match self.next_token(1) {
                Extern => ast.push(self.parse_extern(lexer)),
                _ => break,
            }
        }
        ast
    }

    fn parse_extern(&mut self, lexer: &mut KBuff) -> AST {
        self.consume(lexer);
        let name = match self.next_token(1) {
            Ident(func_name) => func_name,
            x => panic!("Expected function name found {:?}", x),
        };
        self.consume(lexer);

        match self.next_token(1) {
            LParenthesis => {}
            x => panic!("Expected opening paren found {:?}", x),
        }
        self.consume(lexer);

        let mut args = Vec::new();
        match self.next_token(1) {
            Ident(arg) => args.push(arg),
            x => panic!("Expected argument found {:?}", x),
        }
        self.consume(lexer);
        while let Comma = self.next_token(1) {
            self.consume(lexer);
            let mut args = Vec::new();
            match self.next_token(1) {
                Ident(arg) => args.push(arg),
                x => panic!("Expected argument found {:?}", x),
            }
            self.consume(lexer);
        }

        match self.next_token(1) {
            RParenthesis => {}
            x => panic!("Expected opening paren found {:?}", x),
        }
        self.consume(lexer);

        ExternNode(ProtoType::new(name, args))
    }

    // pub fn parse(&mut self, lexer: &mut KBuff) -> Vec<AST> {
    //     self.fill_look_ahead(lexer);
    //     let mut ast = Vec::new();
    //     loop {
    //         match (self.next_token(1), self.next_token(2)) {
    //             (Token::Ident(ref name), Token::Operator(ref op)) if op == "=" => {
    //                 ast.push(self.assigment(lexer, name.to_owned()))
    //             }
    //             _ => {}
    //         }

    //         match self.next_token(1) {
    //             Token::LBracket => ast.push(self.list(lexer)),
    //             Token::Ident(name) => {
    //                 ast.push(self.name(lexer, name));
    //             }
    //             Token::EOF => break,
    //             _ => panic!(),
    //         }
    //     }
    //     ast
    // }
    // fn name(&mut self, lexer: &mut KBuff, name: String) -> AST {
    //     self.consume(lexer);
    //     AST::Element(name)
    // }
    // fn assigment(&mut self, lexer: &mut KBuff, ident: String) -> AST {
    //     self.consume(lexer);
    //     self.consume(lexer);
    //     match self.next_token(1) {
    //         Token::Ident(name) => {
    //             self.consume(lexer);
    //             AST::Assignment(ident, Token::Operator("=".to_owned()), name)
    //         }
    //         _ => panic!(),
    //     }
    // }

    // fn list(&mut self, lexer: &mut KBuff) -> AST {
    //     self.consume(lexer);
    //     let mut ast = Vec::new();
    //     self.elements(lexer, &mut ast);
    //     match self.next_token(1) {
    //         Token::RBracket => {}
    //         _ => panic!("Expected closing"),
    //     }
    //     self.consume(lexer);
    //     AST::List(Token::LBracket, ast, Token::RBracket)
    // }

    // fn elements(&mut self, lexer: &mut KBuff, ast: &mut Vec<AST>) {
    //     match self.next_token(1) {
    //         Token::Ident(name) => {
    //             self.consume(lexer);
    //             ast.push(AST::Element(name.to_owned()))
    //         }
    //         Token::RBracket => return,
    //         x => panic!("Expected Name found {:?}", x),
    //     }
    //     while let Token::Comma = self.next_token(1) {
    //         self.consume(lexer);
    //         match self.next_token(1) {
    //             Token::LBracket => ast.push(self.list(lexer)),
    //             Token::Ident(name) => ast.push(self.name(lexer, name)),
    //             Token::RBracket => return,
    //             x => panic!("Expect Comma: {:?}", x),
    //         };
    //     }
    // }
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
