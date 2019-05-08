use std::fmt::{Debug, Formatter, Result};
use std::str::Chars;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Def,
    Extern,
    Delimiter,
    LParenthesis,
    RParenthesis,
    LBracket,
    RBracket,
    Comma,
    Comment,
    Ident(String),
    String(String),
    Numeric(f64),
    Operator(String),
    EOF,
}

pub struct Tok {
    token: Token,
    lexeme: String,
    line: usize,
}

impl Debug for Tok {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", &self.to_string())
    }
}

impl ToString for Tok {
    fn to_string(&self) -> String {
        format!(
            "<| type: {:?} + raw: {:?} + line: {:?} |>",
            self.token, self.lexeme, self.line
        )
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
                ',' => Token::Comma,
                '[' => Token::LBracket,
                ']' => Token::RBracket,
                '(' => Token::LParenthesis,
                ')' => Token::RParenthesis,
                ';' => Token::Delimiter,
                '\0' => break,
                _ => panic!("Error found {:?}", cur),
            };
            self.consume();
            return token;
        }
        Token::EOF
    }

    #[inline]
    fn numeric(&mut self) -> Token {
        let mut token = String::new();
        while let Some(cur) = self.cur {
            self.consume();
            // Floating point number.
            if cur == '.' && !self.peek().is_numeric() {
                // TODO: This would be a function call on a number, handle accordingly.
                // Ex: 144.sqrt()
                break;
            } else if cur == '.' {
                token.push(cur);
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

            token.push(cur);
        }

        Token::Numeric(token.parse().unwrap())
    }

    #[inline]
    fn ident(&mut self) -> Token {
        let mut token = String::new();
        while let Some(cur) = self.cur {
            if cur.is_whitespace() {
                break;
            }

            if !cur.is_alphanumeric() && cur != '_' {
                self.cur = Some(cur);
                break;
            }

            token.push(cur);
            self.consume();
        }
        match token.as_str() {
            "def" => Token::Def,
            "extern" => Token::Extern,
            _ => Token::Ident(token),
        }
    }

    #[inline]
    fn string(&mut self) -> Token {
        self.consume();
        let mut token = String::new();
        loop {
            if self.peek() == '"' {
                break;
            } else if self.peek() == '\0' {
                panic!("Missing end of string literal");
            }
            token.push(self.cur.unwrap());
            self.consume();
        }
        self.consume();
        Token::String(token)
    }

    #[inline]
    fn op(&mut self, cur: char) -> Token {
        self.consume();
        let mut token = String::new();
        token.push(cur);
        match (cur, self.peek()) {
            ('=', '=') => token.push('='),
            ('!', '=') => token.push('='),
            ('>', '=') => token.push('='),
            ('<', '=') => token.push('='),
            _ => return Token::Operator(token),
        }

        self.consume();
        Token::Operator(token)
    }

    #[inline]
    fn op_or_comment(&mut self, cur: char) -> Token {
        self.consume();
        let mut token = String::new();
        token.push(cur);
        match self.peek() {
            '/' => token.push('/'),
            _ => return Token::Operator(token),
        }
        loop {
            if let Some('\n') | Some('\0') = self.consume() {
                break;
            }
        }
        Token::Comment
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
        if token == Token::EOF {
            return None;
        }
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(tok, Token::Def);
        let mut buf = KBuff::new("foo");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Ident("foo".to_string()));

        let mut buf = KBuff::new("extern");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Extern);
        let mut buf = KBuff::new(",");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Comma);
        let mut buf = KBuff::new(";");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Delimiter);
        let mut buf = KBuff::new("(");
        let tok = buf.next_token();
        assert_eq!(tok, Token::LParenthesis);
        let mut buf = KBuff::new(")");
        let tok = buf.next_token();
        assert_eq!(tok, Token::RParenthesis);
        let mut buf = KBuff::new("[");
        let tok = buf.next_token();
        assert_eq!(tok, Token::LBracket);
        let mut buf = KBuff::new("]");
        let tok = buf.next_token();
        assert_eq!(tok, Token::RBracket);
    }

    #[test]
    fn test_parse_consecutive_tokens() {
        let mut buf = KBuff::new("def foo(x, y) extern, ; ()[]");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Def);
        let tok = buf.next_token();
        assert_eq!(tok, Token::Ident("foo".to_string()));
        let tok = buf.next_token();
        assert_eq!(tok, Token::LParenthesis);
        let tok = buf.next_token();
        assert_eq!(tok, Token::Ident("x".to_string()));
        let tok = buf.next_token();
        assert_eq!(tok, Token::Comma);
        let tok = buf.next_token();
        assert_eq!(tok, Token::Ident("y".to_string()));
        let tok = buf.next_token();
        assert_eq!(tok, Token::RParenthesis);
        let tok = buf.next_token();
        assert_eq!(tok, Token::Extern);
        let tok = buf.next_token();
        assert_eq!(tok, Token::Comma);
        let tok = buf.next_token();
        assert_eq!(tok, Token::Delimiter);
        let tok = buf.next_token();
        assert_eq!(tok, Token::LParenthesis);
        let tok = buf.next_token();
        assert_eq!(tok, Token::RParenthesis);
        let tok = buf.next_token();
        assert_eq!(tok, Token::LBracket);
        let tok = buf.next_token();
        assert_eq!(tok, Token::RBracket);
    }

    #[test]
    fn test_parse_num() {
        let mut buf = KBuff::new("10");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Numeric(10.));
        let mut buf = KBuff::new("20");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Numeric(20.));
        let mut buf = KBuff::new("20.");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Numeric(20.));
        let mut buf = KBuff::new("0.20");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Numeric(0.20));
        let mut buf = KBuff::new("23.4");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Numeric(23.4));
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
        assert_eq!(tok, Token::Operator("+".to_string()));
        let mut buf = KBuff::new("-");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Operator("-".to_string()));
        let mut buf = KBuff::new("*");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Operator("*".to_string()));
        let mut buf = KBuff::new("/");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Operator("/".to_string()));
        let mut buf = KBuff::new("=");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Operator("=".to_string()));
    }

    #[test]
    fn test_parse_string() {
        let mut buf = KBuff::new("\"HelloWorld\"");
        let tok = buf.next_token();
        assert_eq!(tok, Token::String("HelloWorld".to_owned()));

        let mut buf = KBuff::new("def hello_world() \"HelloWorld\"");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Def);
        let tok = buf.next_token();
        assert_eq!(tok, Token::Ident("hello_world".to_owned()));
        let tok = buf.next_token();
        assert_eq!(tok, Token::LParenthesis);
        let tok = buf.next_token();
        assert_eq!(tok, Token::RParenthesis);
        let tok = buf.next_token();
        assert_eq!(tok, Token::String("HelloWorld".to_owned()));
    }

    #[test]
    fn test_parse_mutli_char_ops() {
        let mut buf = KBuff::new("!=");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Operator("!=".to_string()));
        let mut buf = KBuff::new("==");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Operator("==".to_string()));

        let mut buf = KBuff::new("1 != 2");
        let tok = buf.next_token();
        assert_eq!(tok, Token::Numeric(1.));
        let tok = buf.next_token();
        assert_eq!(tok, Token::Operator("!=".to_string()));
        let tok = buf.next_token();
        assert_eq!(tok, Token::Numeric(2.));
    }
}
