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
    EndToken,                        // .
    PredicateListToken,              // ;
    ObjectListToken,                 // ,
    Comment(String),                 // # comment
    Unknown(Option<String>),         // unknown token
}

pub fn tokenize(line: &str, line_num: u32) -> Vec<Lexeme> {
    let mut tokens = Vec::new();
    let mut itr = line.chars().into_iter().enumerate().peekable();

    while let Some((offset, c)) = itr.peek() {
        match c {
            '<' => {
                let iri = read_iri(&mut itr);
                tokens.push(iri);
            }
            '"' => {
                let literal = read_literal(&mut itr);

                match literal {
                    Lexeme::Literal(_) => tokens.push(literal),
                    Lexeme::LangLiteral(_, _) => tokens.push(literal),
                    Lexeme::DataTypeLiteral(_, _) => tokens.push(literal),
                    Lexeme::Unknown(str) => tokens.push(Lexeme::Unknown(str)),
                    _ => tokens.push(Lexeme::Unknown(None)),
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
                                        tokens.push(Lexeme::Unknown(Some(token)));
                                    }
                                }
                            }

                            false => tokens.push(Lexeme::Unknown(Some(token))),
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
                                tokens.push(Lexeme::Unknown(Some(token)));
                            }
                        }
                    }
                    _ => {
                        tokens.push(Lexeme::Unknown(Some(token.to_string())));
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
                    false => tokens.push(Lexeme::Unknown(Some(token.to_string()))),
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
        (_, _) => Lexeme::Unknown(Some(iri)),
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

fn read_literal(itr: &mut Peekable<Enumerate<Chars>>) -> Lexeme {
    let literal = read_literal_value(itr);

    match literal {
        Some(literal) => {
            let current_char = itr.peek();

            match current_char {
                Some((_, '@')) => {
                    itr.next();

                    let language_tag = read_token(itr);

                    Lexeme::LangLiteral(literal, language_tag)
                }
                Some((_, '^')) => {
                    itr.nth(1);

                    let data_type = read_iri(itr);

                    match data_type {
                        Lexeme::Iri(iri) => Lexeme::DataTypeLiteral(literal, iri),
                        _ => Lexeme::Unknown(Some(literal)),
                    }
                }
                _ => Lexeme::Literal(literal),
            }
        }
        None => Lexeme::Unknown(None),
    }
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
