use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

    parse_line(&contents);

    Ok(())
}

fn parse_iri(input: &str) -> Option<String> {
    let mut iri = String::new();
    let mut end_token = false;
    let start_token = input.chars().nth(0);

    match start_token {
        Some('<') => {
            for c in input[1..].chars() {
                if c == '>' {
                    end_token = true;
                    break;
                } else {
                    iri.push(c);
                }
            }
        }
        _ => {}
    }

    match end_token {
        true => Some(iri),
        false => None,
    }
}

fn parse_line(input: &str) -> () {
    let mut current_offset: usize = 0;
    let mut remaining_str = &input[current_offset..];

    while current_offset < input.len() {
        current_offset += count_whitespace(remaining_str);

        remaining_str = &input[current_offset..];

        let start_token = remaining_str.chars().nth(0);

        match start_token {
            Some('<') => {
                let iri = parse_iri(remaining_str);

                match iri {
                    Some(ref parsed_iri) => {
                        current_offset += parsed_iri.chars().count() + 2;
                    }
                    _ => {}
                }

                println!("IRI: {}", iri.unwrap_or(String::from("NOT FOUND")));
            }
            Some('.') => {
                current_offset += 1;
            }
            _ => {}
        }
    }
}

fn count_whitespace(input: &str) -> usize {
    let mut offset = 0;

    for c in input.chars() {
        match c {
            ' ' => offset += 1,
            '\t' => offset += 1,
            _ => break,
        }
    }

    offset
}
