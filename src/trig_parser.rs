use std::{
    collections::HashSet,
    iter::{Enumerate, Peekable},
    str::Chars,
    usize,
};

use crate::trig_parser::lexer::{read_iri_lexeme, Token};

use crate::log::*;

struct ParsedToken<T> {
    token: T,
    consumed_chars: u32,
}

struct TrigParser {
    prefixes: HashSet<String>,
}

impl TrigParser {
    fn new() -> TrigParser {
        TrigParser {
            prefixes: HashSet::new(),
        }
    }
}

pub fn parse_line(input: &str, line_num: u32) -> () {
    let mut line_chars = input.chars().into_iter().enumerate().peekable();

    while let Some((offset, c)) = line_chars.next() {
        match c {
            ' ' | '\t' => {
                // skip whitespace
            }
            '<' => {
                // parse IRI

                let iri = line_chars
                    .by_ref()
                    .map(|(_, c)| c)
                    .take_while(|x| *x != '>')
                    .collect::<String>();

                println!("IRI: {}", iri);
            }
            '"' => {
                // parse literal
                let mut escaped = false;
                let collect_literal = |c: &char| -> bool {
                    if *c == '"' && !escaped {
                        return false;
                    } else if *c == '\\' {
                        escaped = true;
                    } else {
                        escaped = false;
                    }

                    return true;
                };

                let literal = line_chars
                    .by_ref()
                    .map(|(_, c)| c)
                    .take_while(collect_literal)
                    .collect::<String>();

                println!("Literal: {}", literal);

                // check for language tag or datatype
                let next_char = line_chars.peek();
                let not_token_end = |x: &char| *x != ' ' && *x != '\t' && *x != ',' && *x != '.';
                match next_char {
                    Some((_, '@')) => {
                        // language tag

                        let lang_tag = line_chars
                            .by_ref()
                            .map(|(_, c)| c)
                            .take_while(not_token_end)
                            .collect::<String>();

                        println!("Language tag: {}", lang_tag);
                    }
                    Some((_, '^')) => {
                        // datatype
                        let datatype = line_chars
                            .by_ref()
                            .map(|(_, c)| c)
                            .take_while(not_token_end)
                            .collect::<String>();

                        println!("Datatype: {}", datatype);
                    }
                    _ => {}
                }
            }
            '@' => {
                let token = line_chars
                    .by_ref()
                    .map(|(_, c)| c)
                    .take_while(|x| x.is_alphabetic())
                    .collect::<String>();
                let parsed_token = &String::from(format!("{c}{token}"));

                match parsed_token.as_str() {
                    "@prefix" => {
                        let key = &read_iri_lexeme(&mut line_chars);
                        let value = &read_iri_lexeme(&mut line_chars);

                        match (key, value) {
                            (Token::PrefixIri(key), Token::RelativeOrAbsoluteIri(value)) => {
                                println!("@prefix: {} -> {}", key, value);
                            }
                            _ => {
                                log_error(
                                    format!(
                                        "Expected prefix IRI and relative or absolute IRI, found {:?} and {:?}",
                                        key, value
                                    ),
                                    input.to_string(),
                                    parsed_token.to_string(),
                                    line_num,
                                    offset,
                                );
                            }
                        }
                    }
                    _ => {}
                }

                // log_todo(
                //     format! {"parse @{token}"},
                //     input.to_string(),
                //     parsed_token.to_string(),
                //     line_num,
                //     offset,
                // );
            }
            c => {
                let remaining_line =
                    String::from(c) + &line_chars.by_ref().map(|(_, c)| c).collect::<String>();

                // println!("Remaining line: '{}' -> {}", remaining_line, offset);
                // println!("Offending line: '{}'", input.to_string());

                log_error(
                    format! {"Unexpected token '{c}'"},
                    input.to_string(),
                    remaining_line.to_string(),
                    line_num,
                    offset,
                );
            }
        }
    }
}

// Return the next word in the line
pub fn next_word(itr: &mut Peekable<Enumerate<Chars>>) -> Option<String> {
    let mut word = String::new();
    let mut skip_whitespace = true;

    while let Some((_, c)) = itr.peek() {
        if c.is_whitespace() {
            if skip_whitespace {
                itr.next();
                continue;
            } else {
                break;
            }
        }

        word.push(*c);
        skip_whitespace = false;
        itr.next();
    }

    if word.len() > 0 {
        return Some(word);
    }

    return None;
}

#[cfg(test)]
mod tests {
    use crate::trig_parser::lexer::{read_iri_lexeme, Token};

    use super::*;

    #[test]
    fn test_next_word_simple() {
        let mut itr = "hello world".chars().enumerate().peekable();

        assert_eq!(next_word(&mut itr), Some(String::from("hello")));
        assert_eq!(next_word(&mut itr), Some(String::from("world")));
    }

    #[test]
    fn test_next_word_complex() {
        let mut itr = "      hello  world         ".chars().enumerate().peekable();

        assert_eq!(next_word(&mut itr), Some(String::from("hello")));
        assert_eq!(next_word(&mut itr), Some(String::from("world")));
    }

    #[test]
    fn test_next_word_all_whitespace() {
        let mut itr = "               ".chars().enumerate().peekable();

        assert_eq!(next_word(&mut itr), None);
    }

    #[test]
    fn test_lexer_absolute_or_relative_iri_lexeme() {
        let mut itr = "<foo>".chars().enumerate().peekable();

        assert_eq!(
            read_iri_lexeme(&mut itr),
            Token::RelativeOrAbsoluteIri(String::from("foo"))
        );

        let mut itr = "<http://example.org/#batman>"
            .chars()
            .enumerate()
            .peekable();

        assert_eq!(
            read_iri_lexeme(&mut itr),
            Token::RelativeOrAbsoluteIri(String::from("http://example.org/#batman"))
        );

        let mut itr = "     <foo>       ".chars().enumerate().peekable();

        assert_eq!(
            read_iri_lexeme(&mut itr),
            Token::RelativeOrAbsoluteIri(String::from("foo"))
        );

        let mut itr = "<foo".chars().enumerate().peekable();

        assert_eq!(read_iri_lexeme(&mut itr), Token::Unknown(None));

        let mut itr = "<foo bar>".chars().enumerate().peekable();

        assert_eq!(read_iri_lexeme(&mut itr), Token::Unknown(None));
    }

    #[test]
    fn test_lexer_prefix_iri_lexeme() {
        let mut itr = "foo:bar ".chars().enumerate().peekable();

        assert_eq!(
            read_iri_lexeme(&mut itr),
            Token::PrefixIri(String::from("foo:bar"))
        );

        let mut itr = ":foobar ".chars().enumerate().peekable();

        assert_eq!(
            read_iri_lexeme(&mut itr),
            Token::PrefixIri(String::from(":foobar"))
        );

        let mut itr = "foobar: ".chars().enumerate().peekable();

        assert_eq!(
            read_iri_lexeme(&mut itr),
            Token::PrefixIri(String::from("foobar:"))
        );

        let mut itr = "foo:bar,".chars().enumerate().peekable();

        assert_eq!(
            read_iri_lexeme(&mut itr),
            Token::PrefixIri(String::from("foo:bar"))
        );

        let mut itr = "foobar".chars().enumerate().peekable();

        assert_eq!(read_iri_lexeme(&mut itr), Token::Unknown(None));
    }
}

mod lexer {
    use std::{
        iter::{Enumerate, Peekable},
        str::Chars,
        usize,
    };

    #[derive(PartialEq, Debug)]
    pub enum Token {
        RelativeOrAbsoluteIri(String),
        PrefixIri(String),
        Unknown(Option<String>),
    }

    pub fn read_iri_lexeme(itr: &mut Peekable<Enumerate<Chars>>) -> Token {
        let mut lexeme = String::new();

        skip_whitespace(itr);

        match (itr.peek()) {
            Some((_, '<')) => {
                // absolute IRI
                let lexeme = read_absolute_or_relative_iri_lexeme(itr);

                match lexeme {
                    Some(Token::RelativeOrAbsoluteIri(lexeme)) => {
                        return Token::RelativeOrAbsoluteIri(lexeme);
                    }
                    _ => return Token::Unknown(None),
                }
            }
            Some((_, _)) => {
                // relative IRI
                let lexeme = read_prefix_iri_lexeme(itr);

                match lexeme {
                    Some(Token::PrefixIri(lexeme)) => {
                        return Token::PrefixIri(lexeme);
                    }
                    _ => return Token::Unknown(None),
                }
            }
            _ => {
                // unknown
                return Token::Unknown(None);
            }
        }
    }

    fn read_absolute_or_relative_iri_lexeme(itr: &mut Peekable<Enumerate<Chars>>) -> Option<Token> {
        let mut lexeme = String::new();
        let mut found_end_token = false;
        let mut found_whitespace = false;

        let start_token = itr.next();

        match start_token {
            Some((_, c)) if c != '<' => return None,
            _ => {}
        }

        while let Some((_, c)) = itr.next() {
            if c == '>' {
                found_end_token = true;
                break;
            } else if c.is_whitespace() {
                found_whitespace = true;
                break;
            }

            lexeme.push(c);
        }

        match (found_end_token, found_whitespace) {
            (true, false) => Some(Token::RelativeOrAbsoluteIri(lexeme)),
            _ => None,
        }
    }

    fn read_prefix_iri_lexeme(itr: &mut Peekable<Enumerate<Chars>>) -> Option<Token> {
        let mut lexeme = String::new();
        let mut colon_token = false;
        let mut found_end_token = false; // ' ' | ',' coma is used for object list

        while let Some((_, c)) = itr.peek() {
            if c == &':' {
                colon_token = true;
            } else if c.is_whitespace() || c == &',' {
                found_end_token = true;
                break;
            }

            lexeme.push(*c);
            itr.next();
        }

        match (colon_token, found_end_token) {
            (true, true) => Some(Token::PrefixIri(lexeme)),
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
}
