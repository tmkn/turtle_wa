use std::{
    collections::HashSet,
    iter::{Enumerate, Peekable},
    str::Chars,
    usize,
};

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
                let parsed_token = String::from(c) + &token;

                log_todo(
                    format! {"parse @{token}"},
                    input.to_string(),
                    parsed_token.to_string(),
                    line_num,
                    offset,
                );
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
}
