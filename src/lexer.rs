// here we go again

use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

#[derive(PartialEq, Debug)]
pub enum Lexeme {
    Iri(String),                     // <http://example.com>
    PrefixedIri(String),             // ex:foo
    A,                               // a -> rdf:type
    Prefix(String, String),          // @prefix | PREFIX
    Base(String),                    // @base | BASE
    Literal(String),                 // "literal"
    LangLiteral(String, String),     // "literal"@en
    DataTypeLiteral(String, String), // "literal"^^<iri>
    MultilineLiteral(String),        // """literal"""
    EndToken,                        // .
    PredicateListToken,              // ;
    ObjectListToken,                 // ,
    Comment(String),                 // # comment
    Unknown(String),                 // unknown token
}

pub struct LexerContext {
    pub parse_multiline: bool,
    pub parsed_multilines: Vec<Lexeme>,
}

impl LexerContext {
    pub fn new() -> LexerContext {
        LexerContext {
            parse_multiline: false,
            parsed_multilines: Vec::new(),
        }
    }
}

pub fn tokenize(line: &str, line_num: u32, context: &mut LexerContext) -> Vec<Lexeme> {
    let mut tokens = Vec::new();
    let mut itr = line.chars().into_iter().enumerate().peekable();

    if context.parse_multiline {
        match read_multiline_part(&mut itr) {
            (multiline, true) => {
                context
                    .parsed_multilines
                    .push(Lexeme::MultilineLiteral(multiline));

                let multiline_str = context
                    .parsed_multilines
                    .iter()
                    .map(|line| match line {
                        Lexeme::MultilineLiteral(line) => line.clone(),
                        _ => panic!("Unexpected lexeme in multiline literal"),
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                tokens.push(Lexeme::MultilineLiteral(multiline_str));

                // reset context
                context.parse_multiline = false;
                context.parsed_multilines.clear();
            }
            (multiline, false) => {
                context
                    .parsed_multilines
                    .push(Lexeme::MultilineLiteral(multiline));

                context.parse_multiline = true;
            }
        };
    }

    while let Some((offset, c)) = itr.peek() {
        match c {
            '<' => {
                let iri = read_iri(&mut itr);
                tokens.push(iri);
            }
            '"' => {
                let result = read_literal(&mut itr, context);

                match result {
                    Some(lexeme) => tokens.push(lexeme),
                    _ => {}
                }
            }
            '@' | 'P' | 'B' => {
                let token = read_token(&mut itr);

                match token.as_str() {
                    "@prefix" | "PREFIX" => {
                        skip_whitespace(&mut itr);

                        let (token, with_colon) = read_prefix(&mut itr);

                        match with_colon {
                            true => {
                                skip_whitespace(&mut itr);

                                let iri = read_iri(&mut itr);

                                match iri {
                                    Lexeme::Iri(iri) => {
                                        tokens.push(Lexeme::Prefix(token, iri));
                                    }
                                    _ => {
                                        tokens.push(Lexeme::Unknown(token));
                                    }
                                }
                            }

                            false => tokens.push(Lexeme::Unknown(token)),
                        }
                    }
                    "@base" | "BASE" => {
                        skip_whitespace(&mut itr);

                        let base_iri = read_base(&mut itr);

                        match base_iri {
                            Some(iri) => {
                                tokens.push(Lexeme::Base(iri));
                            }
                            _ => {
                                tokens.push(Lexeme::Unknown(token));
                            }
                        }
                    }
                    _ => {
                        tokens.push(Lexeme::Unknown(token.to_string()));
                    }
                }
            }
            '.' => {
                tokens.push(Lexeme::EndToken);
                itr.next();
            }
            ';' => {
                tokens.push(Lexeme::PredicateListToken);
                itr.next();
            }
            ',' => {
                tokens.push(Lexeme::ObjectListToken);
                itr.next();
            }
            'a' => {
                tokens.push(Lexeme::A);
                itr.next();
            }
            ' ' | '\t' => {
                itr.next();
            }
            '#' => {
                let comment = read_comment(&mut itr);

                tokens.push(comment);
            }
            _ => {
                let token = read_token(&mut itr);

                match is_prefixed_uri(&token) {
                    true => tokens.push(Lexeme::PrefixedIri(token.to_string())),
                    false => tokens.push(Lexeme::Unknown(token.to_string())),
                }
            }
        }

        skip_whitespace(&mut itr);
    }

    tokens
}

fn read_iri(itr: &mut Peekable<Enumerate<Chars>>) -> Lexeme {
    let mut iri = String::new();
    let mut found_start = false;
    let mut found_end = false;

    while let Some((_, c)) = itr.peek() {
        match c {
            '<' => {
                itr.next();
                found_start = true;
            }
            '>' => {
                itr.next();
                found_end = true;
                break;
            }
            _ => {
                iri.push(*c);
                itr.next();
            }
        }
    }

    match (found_start, found_end) {
        (true, true) => Lexeme::Iri(iri),
        (_, _) => Lexeme::Unknown(iri),
    }
}

fn is_prefixed_uri(token: &str) -> bool {
    let mut colons: Vec<(usize, char)> = Vec::new();
    let itr = token.chars().enumerate();

    for (offset, c) in itr {
        if let ':' = c {
            colons.push((offset, c));
        }
    }

    let (first, second) = (colons.first(), colons.get(1));

    matches!((first, second), (Some((_, _)), None) if !token.ends_with(':'))
}

fn read_literal(
    itr: &mut Peekable<Enumerate<Chars>>,
    context: &mut LexerContext,
) -> Option<Lexeme> {
    let literal = read_literal_value(itr);

    match literal {
        Some(literal) => {
            let current_char = itr.peek();

            match current_char {
                Some((_, '@')) => {
                    itr.next();

                    let language_tag = read_token(itr);

                    Some(Lexeme::LangLiteral(literal, language_tag))
                }
                Some((_, '^')) => {
                    itr.nth(1);

                    let data_type = read_iri(itr);

                    match data_type {
                        Lexeme::Iri(iri) => Some(Lexeme::DataTypeLiteral(literal, iri)),
                        _ => Some(Lexeme::Unknown(literal)),
                    }
                }
                Some((_, '\"')) => {
                    let (multiline_part, end) = read_multiline_part(itr);

                    match end {
                        true => {
                            context.parse_multiline = false;
                            context.parsed_multilines = Vec::new();
                            Some(Lexeme::MultilineLiteral(multiline_part))
                        }
                        false => {
                            context.parse_multiline = true;
                            context
                                .parsed_multilines
                                .push(Lexeme::MultilineLiteral(multiline_part));
                            None
                        }
                    }
                }
                _ => Some(Lexeme::Literal(literal)),
            }
        }
        None => None,
    }
}

//  read until end of triple quotes or end of line
fn read_multiline_part(itr: &mut Peekable<Enumerate<Chars>>) -> (String, bool) {
    let mut multi_line_part = String::new();
    let mut consecutive_quotes = 0;

    // when called 2 quotes have already been read, check for 3rd to skip
    match itr.peek() {
        Some((_, '\"')) => {
            itr.next();
        }
        _ => {}
    }

    while let Some((_, c)) = itr.peek() {
        match c {
            '\"' => {
                multi_line_part.push(*c);
                itr.next();
                consecutive_quotes += 1;

                if consecutive_quotes == 3 {
                    break;
                }
            }
            _ => {
                multi_line_part.push(*c);
                itr.next();
                consecutive_quotes = 0;
            }
        }
    }

    (
        multi_line_part.trim_end_matches("\"\"\"").to_string(),
        consecutive_quotes == 3,
    )
}

fn read_literal_value(itr: &mut Peekable<Enumerate<Chars>>) -> Option<String> {
    let mut literal = String::new();
    let mut found_end = false;
    let mut is_escaped_quote = false;

    let c = itr.next();

    match c {
        Some((_, '"')) => {}
        _ => return None,
    }

    while let Some((_, c)) = itr.peek() {
        match c {
            '"' if !is_escaped_quote => {
                itr.next();
                found_end = true;
                break;
            }
            '\\' => {
                literal.push(*c);
                itr.next();

                // check if quote is escaped
                let next_char = itr.peek();
                is_escaped_quote = match next_char {
                    Some((_, '"')) => true,
                    _ => false,
                }
            }
            _ => {
                literal.push(*c);
                itr.next();
                is_escaped_quote = false;
            }
        }
    }

    match found_end {
        true => Some(literal),
        false => None,
    }
}

// read the next token until a whitespace is found
fn read_token(itr: &mut Peekable<Enumerate<Chars>>) -> String {
    let mut token = String::new();

    while let Some((_, c)) = itr.peek() {
        match c {
            ' ' | ',' => {
                break;
            }
            _ => {
                token.push(*c);
                itr.next();
            }
        }
    }

    token
}

fn read_prefix(itr: &mut Peekable<Enumerate<Chars>>) -> (String, bool) {
    let mut prefix = String::new();
    let mut found_colon = false;

    while let Some((_, c)) = itr.peek() {
        match c {
            ':' => {
                found_colon = true;
                prefix.push(*c);
                itr.next();
                break;
            }
            _ => {
                prefix.push(*c);
                itr.next();
            }
        }
    }

    (prefix, found_colon)
}

fn read_base(itr: &mut Peekable<Enumerate<Chars>>) -> Option<String> {
    let iri = read_iri(itr);

    match iri {
        Lexeme::Iri(iri) => Some(iri),
        _ => None,
    }
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

fn read_comment(itr: &mut Peekable<Enumerate<Chars>>) -> Lexeme {
    let comment: String = itr.skip(1).map(|(_, c)| c).collect();

    Lexeme::Comment(comment)
}
