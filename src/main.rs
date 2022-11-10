use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//use std::time::{Duration, Instant};

pub mod log;
use crate::log::*;

fn main() -> std::io::Result<()> {
    let ttl_path = Path::new("./ttl/simple.ttl");
    let file_result = File::open(ttl_path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => {
            println!("Couldn't find file {}", ttl_path.display());
            return Err(error);
        }
    };

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut num_lines: u32 = 1;
    for line in contents.lines() {
        parse_line(line, num_lines);

        num_lines += 1;
    }

    Ok(())
}

fn parse_iri(input: &str) -> Option<String> {
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

fn parse_line(input: &str, line_num: u32) -> () {
    let mut current_offset: usize = 0;

    while current_offset < input.len() {
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
