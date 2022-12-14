mod structure;

use std::{collections::HashSet, hash::Hash, str::FromStr};

use crate::{
    day14::structure::{find_max_y, parse_paths, Cave, Line, Path},
    helper::{read_lines, read_string, ParseError},
};

#[test]
pub fn test_day14_pt_1() {
    let input: String = read_string(14, false);
    let paths = parse_paths(&input);
    let mut cave = Cave::new(paths);

    let mut units = 0;
    while cave.next() {
        units = units + 1;
        // println!("{}", cave.to_string());
    }
    println!("Part1 score is {:?}", units);
    assert!(false);
}

#[test]
pub fn test_day14_pt_2() {
    let input: String = read_string(14, false);
    let mut paths = parse_paths(&input);
    let max_y = find_max_y(&paths);
    paths.push(Path {
        lines: vec![Line {
            start: (0, max_y + 2),
            end: (2000, max_y + 2),
        }],
    });
    let mut cave = Cave::new(paths);
    println!("{}", cave.to_string());

    let mut units = 0;
    while cave.next() {
        units = units + 1;
        // println!("{}", cave.to_string());
    }
    println!("Part2 score is {:?}", units);
    assert!(false);
}
