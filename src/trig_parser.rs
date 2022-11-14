use std::collections::HashSet;

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
    let mut line_chars = input.chars().into_iter().peekable();
    let mut line_offset = 0;

    while let Some(c) = line_chars.next() {
        match c {
            ' ' | '\t' => {
                // skip whitespace
                line_offset += 1;
            }
            '<' => {
                // parse IRI

                let iri = line_chars
                    .by_ref()
                    .take_while(|x| *x != '>')
                    .collect::<String>();

                println!("IRI: {}", iri);

                line_offset += iri.len() + 2; // +2 for the '<' and '>'
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
                    .take_while(collect_literal)
                    .collect::<String>();

                println!("Literal: {}", literal);

                // check for language tag or datatype
                let next_char = line_chars.peek();
                let not_token_end = |x: &char| *x != ' ' && *x != '\t' && *x != ',' && *x != '.';
                match next_char {
                    Some('@') => {
                        // language tag

                        let lang_tag = line_chars
                            .by_ref()
                            .take_while(not_token_end)
                            .collect::<String>();

                        println!("Language tag: {}", lang_tag);

                        line_offset += lang_tag.len();
                    }
                    Some('^') => {
                        // datatype
                        let datatype = line_chars
                            .by_ref()
                            .take_while(not_token_end)
                            .collect::<String>();

                        println!("Datatype: {}", datatype);

                        line_offset += datatype.len();
                    }
                    _ => {}
                }

                line_offset += literal.len() + 2; // +2 for the '"' and '"'
            }
            '@' => {
                let token = line_chars
                    .by_ref()
                    .take_while(|x| x.is_alphabetic())
                    .collect::<String>();
                let remaining_line = String::from(c) + &token;

                log_todo(
                    format! {"parse @{token}"},
                    input.to_string(),
                    remaining_line.to_string(),
                    line_num,
                );

                line_offset += token.len();
            }
            c => {
                let remaining_line = String::from(c) + &line_chars.by_ref().collect::<String>();

                println!("Remaining line: '{}' - {}", remaining_line, line_offset);

                log_error(
                    format! {"Unexpected token '{c}'"},
                    input.to_string(),
                    remaining_line.to_string(),
                    line_num,
                );

                line_offset += remaining_line.len();
            }
        }
    }
}
