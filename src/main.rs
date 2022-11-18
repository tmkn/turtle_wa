use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
//use std::time::{Duration, Instant};

mod trig_parser;
use crate::trig_parser::*;

mod lexer;
use crate::lexer::*;

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

    let mut num_lines: u32 = 1;
    for line in f.lines() {
        //parse_line(&line.unwrap(), num_lines);
        let tokens = lexer::tokenize(&line.unwrap(), num_lines);

        for token in tokens {
            println!("{:?}", token);
        }

        num_lines += 1;
    }

    Ok(())
}
