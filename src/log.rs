use colored::*;

fn log(
    prefix: String,
    message: String,
    offending_line: String,
    offending_token: String,
    num_line: u32,
    col: usize,
) -> () {
    let position = format!("{}:{}", num_line, col).yellow();

    println!("{position} - {prefix}: {message}");
    println!("");
    println!("{}  {offending_line}", num_line.to_string().on_white());

    let line_num_padding = num_line.to_string().len();
    let underline = "~".repeat(offending_token.len()).red();
    let offset_padding = " ".repeat(col);

    println!(
        "{}  {}{}",
        format!("{:line_num_padding$}", "").on_white(),
        offset_padding,
        underline
    );
    println!();
}

pub fn log_error(
    message: String,
    offending_line: String,
    offending_token: String,
    num_lines: u32,
    col: usize,
) -> () {
    let prefix = "error".to_string().red().to_string();

    log(
        prefix,
        message,
        offending_line,
        offending_token,
        num_lines,
        col,
    );
}

pub fn log_todo(
    message: String,
    offending_line: String,
    offending_token: String,
    num_lines: u32,
    col: usize,
) -> () {
    let prefix = "todo".to_string().bright_blue().to_string();

    log(
        prefix,
        message,
        offending_line,
        offending_token,
        num_lines,
        col,
    );
}
