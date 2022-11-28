use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::time::{Duration, Instant};

mod lexer;
use crate::lexer::*;

mod parser;
use crate::parser::*;

mod log;

fn main() -> std::io::Result<()> {
    let ttl_path = Path::new("./ttl/simple.ttl");
    let file_result = File::open(ttl_path);

    let f = match file_result {
        Ok(file) => file,
        Err(error) => {
            println!("Couldn't find file {}", ttl_path.display());
            return Err(error);
        }
    };
    let f = BufReader::new(f);

    let now = Instant::now();
    let mut triples: Vec<Triple> = Vec::new();
    let mut context = ParseContext::new();
    let mut lexer_context = LexerContext::new();

    let mut num_lines: u32 = 1;
    for line in f.lines() {
        let tokens = lexer::tokenize(&line.unwrap(), num_lines, &mut lexer_context);
        let new_triples = parser::parse(&tokens, &mut context);

        triples.extend(new_triples);

        num_lines += 1;
    }

    for triple in &triples {
        println!("{:?}", triple);
    }

    println!(
        "Parse time: {}ms for {} triples",
        now.elapsed().as_millis(),
        triples.len()
    );

    Ok(())
}
