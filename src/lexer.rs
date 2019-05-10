use std::fmt::{Debug, Formatter, Result};
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Def,
    Extern,
    Delimiter,
    LParenthesis,
    RParenthesis,
    LBracket,
    RBracket,
    Comma,
    Comment { lexeme: String },
    Ident { lexeme: String },
    String { lexeme: String },
    Numeric { lexeme: String },
    Operator { lexeme: String },
    EOF,
}

#[derive(PartialEq, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub line: usize,
}

impl Token {
    fn new(token: TokenType, line: usize) -> Self {
        Token {
            r#type: token,
            line,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", &self.to_string())
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("<| type: {:?} +  line: {:?} |>", self.r#type, self.line)
    }
}

#[derive(Debug)]
pub struct KBuff<'a> {
    pub cur: Option<char>,
    chars: Chars<'a>,
}

impl<'a> KBuff<'a> {
    pub fn new(input: &'a str) -> Self {
        KBuff {
            cur: Some(' '),
            chars: input.chars(),
        }
    }

    pub fn tokenize(self) -> Vec<Token> {
        self.map(|token| token).collect()
    }

    fn consume(&mut self) -> Option<char> {
        self.cur = self.chars.next();
        self.cur
    }

    pub fn next_token(&mut self) -> Token {
        use TokenType::*;
        while let Some(cur) = self.cur {
            if cur.is_whitespace() {
                self.consume();
                continue;
            }

            let token = match cur {
                // Parse complex tokens.
                x if x.is_numeric() => return self.numeric(),
                x if x.is_alphanumeric() => return self.ident(),
                // Parse strings.
                '"' => self.string(),
                // Parse operators.
                '+' => self.op(cur),
                '-' => self.op(cur),
                '*' => self.op(cur),
                '!' => self.op(cur),
                '<' => self.op(cur),
                '>' => self.op(cur),
                '=' => self.op(cur),
                '/' => self.op_or_comment(cur),

                // Parse single tokens.
                ',' => Token::new(Comma, 0),
                '[' => Token::new(LBracket, 0),
                ']' => Token::new(RBracket, 0),
                '(' => Token::new(LParenthesis, 0),
                ')' => Token::new(RParenthesis, 0),
                ';' => Token::new(Delimiter, 0),
                '\0' => break,
                _ => panic!("Error found {:?}", cur),
            };
            self.consume();
            return token;
        }
        Token::new(EOF, 0)
    }

    #[inline]
    fn numeric(&mut self) -> Token {
        let mut lexeme = String::new();
        while let Some(cur) = self.cur {
            self.consume();
            // Floating point number.
            if cur == '.' && self.peek().is_alphabetic() {
                // TODO: This would be a function call on a number, handle accordingly.
                // Ex: 144.sqrt()
                break;
            } else if cur == '.' {
                lexeme.push(cur);
                continue;
            }

            // Finished parsing number.
            if cur.is_whitespace() {
                break;
            }

            // Error only allow numbers for numeric tokens.
            if !cur.is_numeric() {
                panic!("Error: found {:?} when parsing number", cur);
            }

            lexeme.push(cur);
        }

        Token::new(TokenType::Numeric { lexeme }, 0)
    }

    #[inline]
    fn ident(&mut self) -> Token {
        let mut lexeme = String::new();
        while let Some(cur) = self.cur {
            if cur.is_whitespace() {
                break;
            }

            if !cur.is_alphanumeric() && cur != '_' {
                self.cur = Some(cur);
                break;
            }

            lexeme.push(cur);
            self.consume();
        }
        match lexeme.as_str() {
            "def" => Token::new(TokenType::Def, 0),
            "extern" => Token::new(TokenType::Extern, 0),
            _ => Token::new(TokenType::Ident { lexeme }, 0),
        }
    }

    #[inline]
    fn string(&mut self) -> Token {
        self.consume();
        let mut lexeme = String::new();
        loop {
            if self.peek() == '"' {
                break;
            } else if self.peek() == '\0' {
                panic!("Missing end of string literal");
            }
            lexeme.push(self.cur.unwrap());
            self.consume();
        }
        self.consume();
        Token::new(TokenType::String { lexeme }, 0)
    }

    #[inline]
    fn op(&mut self, cur: char) -> Token {
        self.consume();
        let mut lexeme = String::new();
        lexeme.push(cur);
        match (cur, self.peek()) {
            ('=', '=') => lexeme.push('='),
            ('!', '=') => lexeme.push('='),
            ('>', '=') => lexeme.push('='),
            ('<', '=') => lexeme.push('='),
            _ => return Token::new(TokenType::Operator { lexeme }, 0),
        }

        self.consume();
        Token::new(TokenType::Operator { lexeme }, 0)
    }

    #[inline]
    fn op_or_comment(&mut self, cur: char) -> Token {
        self.consume();
        let mut lexeme = String::new();
        lexeme.push(cur);
        match self.peek() {
            '/' => lexeme.push('/'),
            _ => return Token::new(TokenType::Operator { lexeme }, 0),
        }
        loop {
            lexeme.push(self.peek());
            if let Some('\n') | Some('\0') = self.consume() {
                break;
            }
        }
        Token::new(TokenType::Comment { lexeme }, 0)
    }

    #[inline]
    fn peek(&self) -> char {
        self.cur.unwrap_or('\0')
    }
}

impl<'a> Iterator for KBuff<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let token = self.next_token();
        if token.r#type == TokenType::EOF {
            return None;
        }
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;

    #[test]
    fn test_get_next_char() {
        let mut buf = KBuff::new("def");
        assert_eq!(buf.consume().unwrap(), 'd');
        assert_eq!(buf.consume().unwrap(), 'e');
        assert_eq!(buf.consume().unwrap(), 'f');
        assert_eq!(buf.consume(), None);
        let mut buf = KBuff::new("extern");
        assert_eq!(buf.consume().unwrap(), 'e');
        assert_eq!(buf.consume().unwrap(), 'x');
        assert_eq!(buf.consume().unwrap(), 't');
        assert_eq!(buf.consume().unwrap(), 'e');
        assert_eq!(buf.consume().unwrap(), 'r');
        assert_eq!(buf.consume().unwrap(), 'n');
    }

    #[test]
    fn test_parse_tokens() {
        let mut buf = KBuff::new("def");
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(Def, 0));
        let mut buf = KBuff::new("foo");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Ident {
                    lexeme: "foo".to_owned()
                },
                0
            )
        );

        let mut buf = KBuff::new("extern");
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(Extern, 0));
        let mut buf = KBuff::new(",");
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(Comma, 0));
        let mut buf = KBuff::new(";");
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(Delimiter, 0));
        let mut buf = KBuff::new("(");
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(LParenthesis, 0));
        let mut buf = KBuff::new(")");
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(RParenthesis, 0));
        let mut buf = KBuff::new("[");
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(LBracket, 0));
        let mut buf = KBuff::new("]");
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(RBracket, 0));
    }

    #[test]
    fn test_parse_consecutive_tokens() {
        let mut buf = KBuff::new("def foo(x, y) extern, ; ()[]");
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(Def, 0));
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Ident {
                    lexeme: "foo".to_owned()
                },
                0
            )
        );
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(LParenthesis, 0));
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Ident {
                    lexeme: "x".to_owned()
                },
                0
            )
        );
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(Comma, 0));
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Ident {
                    lexeme: "y".to_owned()
                },
                0
            )
        );
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(RParenthesis, 0));
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(Extern, 0));
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(Comma, 0));
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(Delimiter, 0));
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(LParenthesis, 0));
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(RParenthesis, 0));
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(LBracket, 0));
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(RBracket, 0));
    }

    #[test]
    fn test_parse_num() {
        let mut buf = KBuff::new("10");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Numeric {
                    lexeme: "10".to_owned()
                },
                0
            )
        );
        let mut buf = KBuff::new("20");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Numeric {
                    lexeme: "20".to_owned()
                },
                0
            )
        );
        let mut buf = KBuff::new("20.");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Numeric {
                    lexeme: "20.".to_owned()
                },
                0
            )
        );
        let mut buf = KBuff::new("0.20");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Numeric {
                    lexeme: "0.20".to_owned()
                },
                0
            )
        );
        let mut buf = KBuff::new("23.4");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Numeric {
                    lexeme: "23.4".to_owned()
                },
                0
            )
        );
    }

    #[test]
    fn test_invalid_float_number() {
        let mut buf = KBuff::new(".10");
        let result = std::panic::catch_unwind(move || buf.next_token());
        assert!(result.is_err());
        let mut buf = KBuff::new("1k0");
        let result = std::panic::catch_unwind(move || buf.next_token());
        assert!(result.is_err());
        let mut buf = KBuff::new(".1k0");
        let result = std::panic::catch_unwind(move || buf.next_token());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_single_char_ops() {
        let mut buf = KBuff::new("+");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Operator {
                    lexeme: "+".to_owned()
                },
                0
            )
        );
        let mut buf = KBuff::new("-");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Operator {
                    lexeme: "-".to_owned()
                },
                0
            )
        );
        let mut buf = KBuff::new("*");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Operator {
                    lexeme: "*".to_owned()
                },
                0
            )
        );
        let mut buf = KBuff::new("/");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Operator {
                    lexeme: "/".to_owned()
                },
                0
            )
        );
        let mut buf = KBuff::new("=");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Operator {
                    lexeme: "=".to_owned()
                },
                0
            )
        );
    }

    #[test]
    fn test_parse_string() {
        let mut buf = KBuff::new("\"HelloWorld\"");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                String {
                    lexeme: "HelloWorld".to_owned()
                },
                0
            )
        );

        let mut buf = KBuff::new("def hello_world() \"HelloWorld\"");
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(Def, 0));
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Ident {
                    lexeme: "hello_world".to_owned()
                },
                0
            )
        );
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(LParenthesis, 0));
        let tok = buf.next_token();
        assert_eq!(tok, Token::new(RParenthesis, 0));
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                String {
                    lexeme: "HelloWorld".to_owned()
                },
                0
            )
        );
    }

    #[test]
    fn test_parse_mutli_char_ops() {
        let mut buf = KBuff::new("!=");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Operator {
                    lexeme: "!=".to_string()
                },
                0
            )
        );
        let mut buf = KBuff::new("==");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Operator {
                    lexeme: "==".to_string()
                },
                0
            )
        );

        let mut buf = KBuff::new("1 != 2");
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Numeric {
                    lexeme: "1".to_owned()
                },
                0
            )
        );
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Operator {
                    lexeme: "!=".to_string()
                },
                0
            )
        );
        let tok = buf.next_token();
        assert_eq!(
            tok,
            Token::new(
                Numeric {
                    lexeme: "2".to_owned()
                },
                0
            )
        );
    }
}
