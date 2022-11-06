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

    println!("{}", contents);
    println!("{}", count_diamond_brackets(&contents));

    Ok(())
}

fn count_diamond_brackets(input: &str) -> usize {
    let mut count = 0;
    for c in input.chars() {
        if c == '<' {
            count += 1;
        }
    }
    count
}
