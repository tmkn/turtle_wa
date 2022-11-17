// here we go again

use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

pub enum Lexeme {
    Iri(String),
    PrefixedIri(String),
    Prefix(String, String),
    Base(String),
    Literal(String),
    LangLiteral(String, String),
    DataTypeLiteral(String, String),
    EndToken,           // .
    PredicateListToken, // ;
    ObjectListToken,    // ,
    Unknown(Option<String>),
}

pub fn tokenize(line: &str, line_num: u32) -> Vec<Lexeme> {
    let mut tokens = Vec::new();
    let mut itr = line.chars().into_iter().enumerate().peekable();

    while let Some((offset, c)) = itr.next() {
        skip_whitespace(&mut itr);
    }

    tokens
}

fn skip_whitespace(itr: &mut Peekable<Enumerate<Chars>>) {
    while let Some((_, c)) = itr.peek() {
        if c.is_whitespace() {
            itr.next();
        } else {
            break;
        }
    }
}
