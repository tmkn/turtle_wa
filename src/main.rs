use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//use std::time::{Duration, Instant};

mod trig_parser;
use crate::trig_parser::*;

mod log;

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
