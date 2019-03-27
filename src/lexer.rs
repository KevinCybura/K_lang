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
    Ident(String),
    Numeric(f64),
    Operator(String),
    EOF,
}

#[derive(Debug)]
pub struct KBuff<'a> {
    cur: Option<char>,
    chars: Chars<'a>,
}

impl<'a> KBuff<'a> {
    pub fn new(input: &'a str) -> Self {
        KBuff {
            cur: Some(' '),
            chars: input.chars().to_owned(),
        }
    }

    pub fn tokenize(self) -> Vec<Token> {
        self.into_iter().map(|token| token).collect()
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
            // Parse complex tokens.
            match cur {
                x if x.is_numeric() => return self.numeric(),
                x if x.is_alphanumeric() => return self.ident(),
                x if self.is_op(x) => return self.op(),
                _ => {}
            }

            self.consume();
            // Parse single tokens.
            match cur {
                ',' => return Token::Comma,
                '[' => return Token::LBracket,
                ']' => return Token::RBracket,
                '(' => return Token::LParenthesis,
                ')' => return Token::RParenthesis,
                ';' => return Token::Delimiter,
                _ => break,
            }
        }
        Token::EOF
    }

    fn numeric(&mut self) -> Token {
        let mut token = String::new();
        while let Some(cur) = self.cur {
            self.consume();
            if cur.is_whitespace() || cur.is_alphabetic() {
                break;
            }
            token.push(cur);
        }

        Token::Numeric(token.parse().unwrap())
    }

    fn ident(&mut self) -> Token {
        let mut token = String::new();
        while let Some(cur) = self.cur {
            if cur.is_whitespace() {
                break;
            }

            if !cur.is_alphanumeric() {
                self.cur = Some(cur);
                break;
            }

            token.push(cur);
            self.consume();
        }
        match token.as_str() {
            "def" => return Token::Def,
            "extern" => return Token::Extern,
            _ => return Token::Ident(token),
        }
    }

    fn op(&mut self) -> Token {
        let mut token = String::new();
        token.push(self.cur.unwrap());
        self.consume();
        Token::Operator(token)
    }

    fn is_op(&self, op: char) -> bool {
        match op {
            '+' => true,
            '-' => true,
            '*' => true,
            '/' => true,
            _ => false,
        }
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
    }

    #[test]
    fn test_parse_ops() {
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
    }
}
