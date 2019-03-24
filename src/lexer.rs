use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind, Read};

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Def,
    Extern,
    Delimiter,
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident(String),
    Numeric(f64),
    Operator(String),
}

#[derive(Debug)]
struct KBuff {
    cur: char,
    identifier: String,
    file: BufReader<File>,
    file_name: String,
}

impl KBuff {
    fn new(f: String) -> Self {
        KBuff {
            cur: ' ',
            identifier: String::new(),
            file_name: f.clone(),
            file: BufReader::new(File::open(f).unwrap()),
        }
    }
}
impl KBuff {
    fn get_next_char(&mut self) -> Result<char, Error> {
        let mut buf = vec![0; 1];
        if self.file.read(&mut buf).expect("Couldn't read to buffer") != 0 {
            self.cur = String::from_utf8(buf)
                .expect("from_utf8 failed")
                .chars()
                .next()
                .unwrap();
            return Ok(self.cur);
        }

        Err(Error::new(ErrorKind::Other, "EOF"))
    }
}

fn is_op(c: char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        '*' => true,
        '/' => true,
        _ => false,
    }
}

fn parse_token(buf: &mut KBuff) -> Result<Token, Error> {
    // Skip whitespace.
    if buf.cur.is_whitespace() {
        while buf.get_next_char()?.is_whitespace() {}
    }
    let mut ident = String::from("");

    // Handle numbers.
    if buf.cur.is_numeric() {
        ident.push(buf.cur);
        while buf.get_next_char()?.is_numeric() {
            ident.push(buf.cur);
        }
        if buf.cur.is_alphanumeric() {
            eprintln!("Error numbers can't contain characters");
            return Ok(Token::Delimiter);
        }
        return Ok(Token::Numeric(ident.parse().unwrap()));
    }

    // Handle Operators.
    if is_op(buf.cur) {
        ident.push(buf.cur);
        buf.get_next_char()?;
        return Ok(Token::Operator(ident));
    }

    // Handle extras.
    match buf.cur {
        '(' => {
            buf.get_next_char()?;
            return Ok(Token::OpeningParenthesis);
        }
        ')' => {
            buf.get_next_char()?;
            return Ok(Token::ClosingParenthesis);
        }
        ',' => {
            buf.get_next_char()?;
            return Ok(Token::Comma);
        }
        ';' => {
            buf.get_next_char()?;
            return Ok(Token::Delimiter);
        }
        c if is_op(c) => {
            // this is will never be reached but I dont know if I want to change how I handle
            // operators.
            ident.push(buf.cur);
            buf.get_next_char()?;
            return Ok(Token::Operator(ident));
        }
        _ => {}
    }
    ident.push(buf.cur);
    while buf.get_next_char()?.is_alphanumeric() {
        ident.push(buf.cur);
    }

    // Handle Ident, Def or Extern.
    match ident.as_str() {
        "def" => Ok(Token::Def),
        "extern" => Ok(Token::Extern),
        _ => Ok(Token::Ident(ident)),
    }
}

fn with_str<'a>(s: &'a str) -> KBuff {
    let file_name = "temp.k".to_string();
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_name.clone())
        .unwrap();
    writeln!(file, "{}", s).unwrap();
    let file = OpenOptions::new()
        .read(true)
        .open(file_name.clone())
        .unwrap();
    KBuff {
        cur: ' ',
        identifier: String::new(),
        file_name,
        file: BufReader::new(file),
    }
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut buff = with_str(s);
    let mut ret = Vec::new();
    while let Ok(tok) = parse_token(&mut buff) {
        ret.push(tok);
    }
    std::fs::remove_file("temp.k");

    ret
}

pub fn parse(f: String) -> String {
    let mut kbuff = KBuff::new(f);
    while let Ok(tok) = parse_token(&mut kbuff) {
        dbg!(tok);
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use std::fs::OpenOptions;
    use uuid::Uuid;

    impl KBuff {
        fn with_str<'a>(s: &'a str) -> Self {
            let file_name = format!("{:?}.k", Uuid::new_v4().to_string());
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(file_name.clone())
                .unwrap();
            writeln!(file, "{}", s).unwrap();
            let file = OpenOptions::new()
                .read(true)
                .open(file_name.clone())
                .unwrap();
            KBuff {
                cur: ' ',
                identifier: String::new(),
                file_name,
                file: BufReader::new(file),
            }
        }
        fn rewrite<'a>(self, s: &'a str) -> Self {
            KBuff::with_str(s)
        }
    }
    impl Drop for KBuff {
        fn drop(&mut self) {
            match remove_file(self.file_name.clone()) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    #[test]
    fn test_get_next_char() {
        let mut buf = KBuff::with_str("def");
        assert_eq!(buf.get_next_char().unwrap(), 'd');
        assert_eq!(buf.get_next_char().unwrap(), 'e');
        assert_eq!(buf.get_next_char().unwrap(), 'f');
        let mut buf = buf.rewrite("extern");
        assert_eq!(buf.get_next_char().unwrap(), 'e');
        assert_eq!(buf.get_next_char().unwrap(), 'x');
        assert_eq!(buf.get_next_char().unwrap(), 't');
        assert_eq!(buf.get_next_char().unwrap(), 'e');
        assert_eq!(buf.get_next_char().unwrap(), 'r');
        assert_eq!(buf.get_next_char().unwrap(), 'n');
    }

    #[test]
    fn test_parse_tokens() {
        let mut buf = KBuff::with_str("def");
        assert_eq!(parse_token(&mut buf).unwrap(), Token::Def);
        let mut buf = buf.rewrite("foo");
        assert_eq!(
            parse_token(&mut buf).unwrap(),
            Token::Ident("foo".to_string())
        );
        let mut buf = buf.rewrite("extern");
        assert_eq!(parse_token(&mut buf).unwrap(), Token::Extern);
        let mut buf = buf.rewrite(",");
        assert_eq!(parse_token(&mut buf).unwrap(), Token::Comma);
        let mut buf = buf.rewrite(";");
        assert_eq!(parse_token(&mut buf).unwrap(), Token::Delimiter);
        let mut buf = buf.rewrite("(");
        assert_eq!(parse_token(&mut buf).unwrap(), Token::OpeningParenthesis);
        let mut buf = buf.rewrite(")");
        assert_eq!(parse_token(&mut buf).unwrap(), Token::ClosingParenthesis);
    }
    #[test]
    fn test_parse_num() {
        let mut buf = KBuff::with_str("10");
        assert_eq!(parse_token(&mut buf).unwrap(), Token::Numeric(10.0));
        let mut buf = buf.rewrite("20");
        assert_eq!(parse_token(&mut buf).unwrap(), Token::Numeric(20.0));
    }
    #[test]
    fn test_parse_ops() {
        let mut buf = KBuff::with_str("+");
        assert_eq!(
            parse_token(&mut buf).unwrap(),
            Token::Operator("+".to_string())
        );
        let mut buf = buf.rewrite("-");
        assert_eq!(
            parse_token(&mut buf).unwrap(),
            Token::Operator("-".to_string())
        );
        let mut buf = buf.rewrite("*");
        assert_eq!(
            parse_token(&mut buf).unwrap(),
            Token::Operator("*".to_string())
        );
        let mut buf = buf.rewrite("/");
        assert_eq!(
            parse_token(&mut buf).unwrap(),
            Token::Operator("/".to_string())
        );
    }
}