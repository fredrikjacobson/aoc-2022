use std::{
    fs::{self, File},
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
pub fn read_string(day: i32, test: bool) -> String {
    let path = format!(
        "resources/day{}{}.txt",
        day,
        if test { "_test" } else { "" }
    );
    fs::read_to_string(path).unwrap()
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
