use std::{collections::HashSet, hash::Hash, str::FromStr};

use crate::helper::{read_lines, ParseError};

fn parse_range(range_str: &str) -> HashSet<usize> {
    let (from, to) = range_str.split_once("-").unwrap();
    let from = from.parse::<usize>().unwrap();
    let to = to.parse::<usize>().unwrap();
    let range: Vec<usize> = (from..to + 1).collect();
    HashSet::from_iter(range)
}

#[test]
pub fn test_range() {
    let range = parse_range("53-53");
    assert!(range.contains(&53));
}

fn parse_from_to(range_str: &str) -> (usize, usize) {
    let (from, to) = range_str.split_once("-").unwrap();
    let from = from.parse::<usize>().unwrap();
    let to = to.parse::<usize>().unwrap();
    (from, to)
}

#[test]
pub fn test_day4_pt_1() {
    let assignments: Vec<String> = read_lines(4, false);
    let common_items: usize = assignments
        .iter()
        .map(|row| {
            let (first, second) = row.split_once(",").unwrap();
            let first_set = parse_range(first);
            let second_set = parse_range(second);
            (first_set, second_set)
        })
        .filter(fully_contains)
        .count();

    println!("Part1 score is {:?}", common_items);
    assert!(false);
}
#[test]
pub fn test_day4_pt_2() {
    let assignments: Vec<String> = read_lines(4, false);
    let common_items: usize = assignments
        .iter()
        .map(|row| {
            let (first, second) = row.split_once(",").unwrap();
            let first_set = parse_range(first);
            let second_set = parse_range(second);
            (first_set, second_set)
        })
        .filter(overlaps)
        .count();

    println!("Part1 score is {:?}", common_items);
    assert!(false);
}

fn overlaps(sets: &(HashSet<usize>, HashSet<usize>)) -> bool {
    let (first_set, second_set) = sets;
    if (first_set.intersection(&second_set).count() > 0) {
        return true;
    } else {
        return false;
    }
}
fn fully_contains(sets: &(HashSet<usize>, HashSet<usize>)) -> bool {
    let (first_set, second_set) = sets;
    if (first_set.is_superset(&second_set) || second_set.is_superset(&first_set)) {
        return true;
    } else {
        return false;
    }
}
// #[test]
// pub fn test_day4_pt_2() {
//     let rucksacks: Vec<String> = read_lines(4, false);

//     println!("Part1 score is {:?}", scores.iter().sum::<u32>());
//     assert!(false);
// }
