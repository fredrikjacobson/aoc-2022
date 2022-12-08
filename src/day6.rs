use std::{collections::HashSet, hash::Hash, str::FromStr};

use crate::helper::{read_lines, ParseError};

#[test]
pub fn test_day6_pt_1() {
    let lines: Vec<String> = read_lines(6, false);

    let line = lines[0].to_owned();

    let signals = find_window(&line, 4);
    println!("Part1 score is {:?}", signals);
    assert!(false);
}
#[test]
pub fn test_day6_pt_2() {
    let lines: Vec<String> = read_lines(6, false);

    let line = lines[0].to_owned();

    let signals = find_window(&line, 14);
    println!("Part2 score is {:?}", signals);
    assert!(false);
}

fn find_window(line: &str, window_size: usize) -> usize {
    for (i, w) in line
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
    {
        let unique: HashSet<&char> = HashSet::from_iter::<Vec<&char>>(w.into_iter().collect());
        if unique.len() == window_size {
            return i + window_size;
        }
    }

    return 0;
}
