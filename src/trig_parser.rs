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

pub fn parse_iri(input: &str) -> Option<String> {
    let mut iri = String::new();
    let mut end_token = false;
    let start_token = input.chars().next();

    if let Some('<') = start_token {
        for c in input[1..].chars() {
            if c == '>' {
                end_token = true;
                break;
            } else {
                iri.push(c);
            }
        }
    }

    match end_token {
        true => Some(iri),
        false => None,
    }
}

pub fn parse_line(input: &str, line_num: u32) -> () {
    let mut current_offset: usize = 0;

    while current_offset < input.chars().count() {
        let remaining_str = &input[current_offset..];

        let start_token = remaining_str.chars().nth(0);

        match start_token {
            Some(' ') | Some('\t') => {
                // skip whitespace
                current_offset += 1;
            }
            Some('<') => {
                let iri = parse_iri(remaining_str);

                if let Some(ref parsed_iri) = iri {
                    current_offset += parsed_iri.chars().count() + 2;
                }

                println!("IRI: {}", iri.unwrap_or_else(|| String::from("NOT FOUND")));
            }
            Some('"') => {
                log_todo(
                    format! {"parse literal"},
                    input.to_string(),
                    remaining_str.to_string(),
                    line_num,
                );

                current_offset += remaining_str.len();
            }
            Some('#') => {
                log_todo(
                    format! {"parse comment"},
                    input.to_string(),
                    remaining_str.to_string(),
                    line_num,
                );

                current_offset += remaining_str.len();
            }
            Some('.') => {
                current_offset += 1;
            }
            Some(_token) if remaining_str.starts_with("@prefix") => {
                log_todo(
                    format! {"parse @prefix"},
                    input.to_string(),
                    remaining_str.to_string(),
                    line_num,
                );

                current_offset += remaining_str.len();
            }
            Some(token) => {
                log_error(
                    format! {"Unexpected token '{token}'"},
                    input.to_string(),
                    remaining_str.to_string(),
                    line_num,
                );

                current_offset += remaining_str.len();
            }
            _ => {}
        }
    }
}
