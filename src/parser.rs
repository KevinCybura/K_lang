use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read};

#[derive(PartialEq, Debug, Clone)]
enum Token {
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
}

impl KBuff {
    fn new(f: String) -> Self {
        KBuff {
            cur: ' ',
            identifier: String::new(),
            file: BufReader::new(File::open(f).unwrap()),
        }
    }

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

fn is_op(c: &char) -> bool {
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
    if is_op(&buf.cur) {
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
        c if is_op(&c) => {
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

pub fn parse(f: String) -> String {
    let mut kbuff = KBuff::new(f);
    while let Ok(tok) = parse_token(&mut kbuff) {
        dbg!(tok);
    }

    "".to_string()
}
