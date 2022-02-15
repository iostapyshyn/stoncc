use std::str;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Int(i32),
    Sym(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Caret,
    Fac,
    // LBracket,
    // RBracket,
    // LBrace,
    // RBrace,
    // Dot,
    // Percent,
    Eof,
}

impl Token {
    fn from_op(s: &[u8]) -> Self {
        match s[0] {
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Star,
            b'/' => Token::Slash,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'^' => Token::Caret,
            b'!' => Token::Fac,
            // b'[' => Token::LBracket,
            // b']' => Token::RBracket,
            // b'{' => Token::LBrace,
            // b'}' => Token::RBrace,
            // b'.' => Token::Dot,
            // b'%' => Token::Percent,
            _ => panic!("{}", s[0] as char),
        }
    }

    fn from_int(s: &[u8]) -> (Self, usize) {
        let mut i = 0;
        while s.get(i).map_or(false, |c| c.is_ascii_digit()) {
            i += 1
        }

        let num = str::from_utf8(&s[0..i]).unwrap();
        let num = num.parse().unwrap();

        (Self::Int(num), i)
    }

    fn from_symbol(s: &[u8]) -> (Self, usize) {
        let mut i = 0;
        while s.get(i).map_or(false, |c| c.is_ascii_alphanumeric()) {
            i += 1;
        }

        let sym = str::from_utf8(&s[0..i]).unwrap().to_string();

        (Self::Sym(sym), i)
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    peeked: Option<Token>,
    s: &'a [u8],
    i: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        Self {
            peeked: None,
            i: 0,
            s,
        }
    }

    pub fn next(&mut self) -> Token {
        if let Some(t) = self.peeked.take() {
            return t;
        }

        let s = &mut self.s;
        let i = &mut self.i;

        while *i < s.len() {
            let c = s[*i];
            match c {
                b'+' | b'-' |
                b'*' | b'/' |
                b'^' | b'!' |
                b'(' | b')' => {
                    let t = Token::from_op(&s[*i..]);
                    *i += 1;

                    return t;
                }
                b'0'..=b'9' => {
                    let (t, j) = Token::from_int(&s[*i..]);
                    *i += j;

                    return t;
                }
                _ if c.is_ascii_alphabetic() => {
                    let (t, j) = Token::from_symbol(&s[*i..]);
                    *i += j;

                    return t;
                }
                _ if c.is_ascii_whitespace() => *i += 1,
                _ => self.error(),
            };
        }

        Token::Eof
    }

    pub fn peek(&mut self) -> &Token {
        if self.peeked.is_none() {
            self.peeked = Some(self.next());
        }
        self.peeked.as_ref().unwrap()
    }

    fn error(&self) -> ! {
        eprintln!("Syntax error at {}", self.i);
        std::process::exit(1);
    }
}
