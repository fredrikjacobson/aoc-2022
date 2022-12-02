use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_lines(day: i32, test: bool) -> Vec<String> {
    let file = File::open(format!(
        "resources/day{}{}.txt",
        day,
        if test { "_test" } else { "" }
    ))
    .unwrap();
    let reader = BufReader::new(file);
    reader.lines().filter_map(std::io::Result::ok).collect()
}

#[derive(Debug)]
pub struct ParseError {
    details: String,
}

impl ParseError {
    pub fn new(msg: &str) -> ParseError {
        ParseError {
            details: msg.to_string(),
        }
    }
}
