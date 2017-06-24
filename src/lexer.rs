use ast::{WireValue, WireWidth};
use std::str::CharIndices;
use extprim::u128::u128;
use errors::Error;

pub type Spanned<T, E> = Result<(usize, T, usize), E>;

pub type Loc = usize;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Tok<'input> {
    AndAnd, OrOr,
    Equal, NotEqual, GreaterEqual, Greater, LessEqual, Less,
    Assign,
    RightShift, LeftShift,
    Comma, Semicolon,
    Plus, Minus, And, Or, Xor, Times, Divide, Not,
    Constant(WireValue),
    OpenParen, CloseParen, OpenBrace, CloseBrace, OpenBracket, CloseBracket,
    Colon,
    Complement,
    DotDot,
    Wire,
    Const,
    Register,
    Identifier(&'input str),
}

// FIXME: untested code; use this to make good error messages
pub struct FileContents<'input> {
    data: &'input str,
    filenames: Vec<(usize, String)>,
    newlines: Vec<(usize, usize)>,
}

impl<'input> FileContents<'input> {
    pub fn data(&self) -> &'input str { self.data }

    pub fn filename(&self, index: usize) -> &str {
        let index = match self.filenames.binary_search_by_key(&index, |ref x| x.0) {
            Ok(x) => x,
            Err(x) => x - 1
        };
        &self.filenames[index].1
    }

    pub fn line_number_and_bounds(&self, index: usize) -> (usize, usize, usize) {
        let filename = self.filename(index);
        let index = match self.filenames.binary_search_by_key(&index, |ref x| x.0) {
            Ok(x) => x,
            Err(x) => x - 1
        };
        let next_line_loc = if index == self.newlines.len() - 1 { self.data.len() } else { self.newlines[index + 1].0 };
        let cur_line = self.newlines[index];
        (cur_line.1, cur_line.0, next_line_loc)
    }

    pub fn line(&self, index: usize) -> usize {
        self.line_number_and_bounds(index).1
    }

    pub fn file_and_line(&self, index: usize) -> String {
        let filename = self.filename(index);
        let line = self.line(index);
        format!("{}:{}", filename, line)
    }

    pub fn range(&self, start: usize, end: usize) -> String {
        let filename = self.filename(start);
        let start_line = self.line(start);
        let end_line = self.line(start);
        if start_line == end_line {
            format!("{}:{}", filename, start_line)
        } else {
            format!("{}:{}-{}", filename, start_line, end_line)
        }
    }

    pub fn show_region(&self, start: usize, end: usize) -> String {
        let filename = self.filename(start);
        let (begin_line_no, begin_loc, _) = self.line_number_and_bounds(start);
        let (end_line_no, _, end_loc) = self.line_number_and_bounds(start);
        let segment = &self.data[begin_loc..end_loc];
        let mut result = String::new();
        // FIXME: variable width line count
        result.push_str(&format!("     -> {}:{}\n", filename, begin_line_no));
        result.push_str(         "     |\n");
        let mut number = begin_line_no;
        for line in segment.lines() {
            result.push_str(&format!("{:4} | {}\n", number, line));
            number += 1;
        }
        result
    }
}

pub struct Lexer<'input> {
    input: &'input str,
    chars: CharIndices<'input>,
    pending: Option<(usize, char)>,
    last: Option<(usize, char)>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { input: input, chars: input.char_indices(), pending: None, last: None }
    }

    fn internal_next(&mut self) -> Option<(usize, char)> {
        match self.pending {
            Some(x) => {
                debug!("next from unget");
                self.last = self.pending;
                self.pending = None;
            },
            None => {
                self.last = self.chars.next()
            }
        }
        assert_eq!(self.pending, None);
        debug!("next is {:?}", self.last);
        self.last
    }

    fn unget(&mut self) {
        debug!("unget");
        assert!(self.last.is_some());
        assert!(self.pending.is_none());
        self.pending = self.last;
        self.last = None;
    }


    fn peek_char(&mut self) -> Option<char> {
        if let Some((i, c)) = self.internal_next() {
            debug!("peeked at {:?}", c);
            self.unget();
            Some(c)
        } else {
            None
        }
    }

    fn choose_token(&mut self, start: usize, default: Tok<'static>, options: &[(char, Tok<'static>)]) ->  
            Spanned<Tok<'input>, Error> {
        let peeked = self.peek_char();
        for option in options {
            if Some(option.0) == peeked {
                debug!("token {:?}", option.1);
                self.internal_next();
                return Ok((start, option.1, start + 2))
            }
        }
        debug!("token {:?}", default);
        Ok((start, default, start + 1))
    }

    fn extract(&self, start: usize, end: usize) -> &'input str {
        return &self.input[start..end];
    }

    fn get_while<F>(&mut self, start: usize, f: F) -> (usize, &'input str, usize) where F: Fn(char) -> bool {
        let mut last = start;
        while let Some((i, c)) = self.internal_next() {
            if f(c) {
                last = i;
            } else {
                self.unget();
                break;
            }
        }
        (start, self.extract(start, last + 1), last + 1)
    }

    fn expect_or_error<F>(&mut self, f: F) -> Result<char, Error> where F: Fn(char) -> bool {
        if let Some((i, c))  = self.internal_next() {
            if f(c) {
                Ok(c)
            } else {
                Err(Error::LexicalError(i))
            }
        } else {
            Err(Error::LexicalError(usize::max_value()))
        }
    }

    fn expect_peek_not<F>(&mut self, f: F) -> Result<(), Error> where F: Fn(char) -> bool {
        if let Some((i, c))  = self.internal_next() {
            if f(c) {
                Err(Error::LexicalError(i))
            } else {
                self.unget();
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn handle_constant(&mut self, i: usize, c: char) -> Spanned<Tok<'input>, Error> {
        if let Some((i2, c2)) = self.internal_next() {
            match c2 {
                'x' => {
                    let c3 = self.expect_or_error(is_hexadecimal_char)?;
                    let (start_noprefix, hex, end) = self.get_while(i + 2, is_hexadecimal_char);
                    let start = start_noprefix - 2;
                    match u128::from_str_radix(&hex, 16) {
                        Ok(value) => {
                            return Ok((start, Tok::Constant(WireValue::new(value)), end));
                        }
                        Err(_) => {
                            return Err(Error::InvalidConstant(start, end));
                        }
                    }
                },
                'b' => {
                    let c3 = self.expect_or_error(is_binary_char)?;
                    let (start_noprefix, bin, end) = self.get_while(i + 2, is_binary_char);
                    let start = start_noprefix - 2;
                    self.expect_peek_not(is_decimal_char)?; // disallow 0b010112
                    match u128::from_str_radix(&bin, 2) {
                        Ok(value) => {
                            let width = WireWidth::Bits(bin.len() as u8);
                            return Ok((start, Tok::Constant(WireValue::new(value).as_width(width)), end));
                        }
                        Err(_) => {
                            return Err(Error::InvalidConstant(start, end));
                        }
                    }
                }
                '0' ... '9' => {
                    self.unget();
                    let (start, num, end) = self.get_while(i, is_decimal_char);
                    match u128::from_str_radix(&num, 10) {
                        Ok(value) => {
                            return Ok((start, Tok::Constant(WireValue::new(value)), end));
                        }
                        Err(_) => {
                            return Err(Error::InvalidConstant(start, end));
                        }
                    }
                },
                _ => {
                    self.unget();
                    let start = i;
                    let end = i + 1;
                    let num = self.extract(start, end);
                    match u128::from_str_radix(&num, 10) {
                        Ok(value) => {
                            return Ok((start, Tok::Constant(WireValue::new(value)), end));
                        }
                        Err(_) => {
                            return Err(Error::InvalidConstant(start, end));
                        }
                    }
                },
            }
        } else {
            let start = i;
            let end = i + 1;
            let num = self.extract(start, end);
            match u128::from_str_radix(&num, 10) {
                Ok(value) => {
                    return Ok((start, Tok::Constant(WireValue::new(value)), end));
                }
                Err(_) => {
                    return Err(Error::InvalidConstant(start, end));
                }
            }
        }
    }

    fn resolve_identifier(&self, start: Loc, end: Loc) -> Tok<'input> {
        let name = self.extract(start, end);
        match name {
            "wire" => Tok::Wire,
            "const" => Tok::Const,
            "register" => Tok::Register,
            _ => Tok::Identifier(name),
        }
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_start_identifier_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_hexadecimal_char(c: char) -> bool {
    (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')  || (c >= 'A' && c <= 'F')
}

fn is_decimal_char(c: char) -> bool {
    (c >= '0' && c <= '9')
}

fn is_binary_char(c: char) -> bool {
    (c >= '0' && c <= '1')
}

fn is_not_newline(c: char) -> bool {
    c != '\n'
}

fn is_not_star(c: char) -> bool {
    c != '*'
}

fn simple_token<'input>(i: usize, t: Tok<'input>) -> Spanned<Tok<'input>, Error> {
    debug!("simple token {:?}", t);
    Ok((i, t, i + 1))
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((i, c)) = self.internal_next() {
                let cur = (i, c);
                debug!("read {:?}", (i, c));
                if c.is_whitespace() {
                    debug!("skip whitespace");
                    continue;
                } else if is_start_identifier_char(c) {
                    debug!("identifier?");
                    let (start, id, end) = self.get_while(i, is_identifier_char);
                    let the_token = self.resolve_identifier(start, end);
                    return Some(Ok((start, the_token, end)));
                } else if c >= '0' && c <= '9' {
                    debug!("integer constant");
                    return Some(self.handle_constant(i, c));
                } else {
                    let result = match c {
                        '#' => { // comment
                            debug!("# comment");
                            self.get_while(i, is_not_newline);
                            continue;
                        },
                        '/' => {
                            let c2 = self.peek_char();
                            if c2 == Some('/') { // //-style comment
                                debug!("// comment");
                                self.get_while(i, is_not_newline);
                                continue;
                            } else if c2 == Some('*') { // /*-style comment
                                debug!("/* comment");
                                loop {
                                    self.get_while(i, is_not_star);
                                    if self.peek_char() == Some('*') {
                                        self.internal_next();
                                        if self.peek_char() == Some('/') {
                                            self.internal_next();
                                            break;
                                        }
                                    }
                                }
                                if self.peek_char() == None {
                                    return Some(Err(Error::UnterminatedComment(i)));
                                }
                                continue;
                            } else {
                                simple_token(i, Tok::Divide)
                            }
                        },
                        '&' => self.choose_token(i, Tok::And, &[('&', Tok::AndAnd)]),
                        '|' => self.choose_token(i, Tok::Or, &[('|', Tok::OrOr)]),
                        '=' => self.choose_token(i, Tok::Assign, &[('=', Tok::Equal)]),
                        '>' => self.choose_token(i, Tok::Greater, &[('>', Tok::RightShift), ('=', Tok::GreaterEqual)]),
                        '<' => self.choose_token(i, Tok::Less, &[('<', Tok::LeftShift), ('=', Tok::LessEqual)]),
                        '!' => self.choose_token(i, Tok::Not, &[('=', Tok::NotEqual)]),
                        ':' => simple_token(i, Tok::Colon),
                        '~' => simple_token(i, Tok::Complement),
                        ',' => simple_token(i, Tok::Comma),
                        ';' => simple_token(i, Tok::Semicolon),
                        '.' => {
                            if self.peek_char() == Some('.') {
                                self.internal_next();
                                debug!("token: ..");
                                Ok((i, Tok::DotDot, i + 2))
                            } else {
                                debug!("lexical error from ._");
                                Err(Error::LexicalError(i))
                            }
                        },
                        '+' => simple_token(i, Tok::Plus),
                        '-' => simple_token(i, Tok::Minus),
                        '^' => simple_token(i, Tok::Xor),
                        '*' => simple_token(i, Tok::Times),
                        '(' => simple_token(i, Tok::OpenParen),
                        ')' => simple_token(i, Tok::CloseParen),
                        '[' => simple_token(i, Tok::OpenBracket),
                        ']' => simple_token(i, Tok::CloseBracket),
                        '{' => simple_token(i, Tok::OpenBrace),
                        '}' => simple_token(i, Tok::CloseBrace),
                        _ => {
                            debug!("lexical error from unknown character");
                            Err(Error::LexicalError(i))
                        },
                    };
                    return Some(result);
                }
            } else {
                return None;
            }
        }
    }
}
