use std::{cmp::Ordering, collections::HashSet, hash::Hash, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{self, digit1, newline, one_of},
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketData {
    Integer { value: usize },
    List { values: Vec<PacketData> },
}

use super::compare;
use PacketData::*;

#[derive(Debug, Clone)]
pub struct Packet {
    pub left: Vec<PacketData>,
    pub right: Vec<PacketData>,
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match compare(&vec![self.clone()], &vec![other.clone()]) {
            Some(false) => Ordering::Greater,
            Some(true) => Ordering::Less,
            None => Ordering::Equal,
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_list(input: &str) -> Vec<PacketData> {
    let mut stack = Vec::new();
    let mut current: Vec<PacketData> = Vec::new();

    let mut num = "".to_owned();
    for c in input.chars() {
        if c == '[' {
            stack.push(current);
            current = Vec::new();
        } else if c.is_numeric() {
            num = num + &c.to_string();
        } else if c == ',' {
            if !num.is_empty() {
                current.push(Integer {
                    value: num.parse::<usize>().unwrap(),
                });
                num = "".to_owned();
            }
        } else if c == ']' {
            if !num.is_empty() {
                current.push(Integer {
                    value: num.parse::<usize>().unwrap(),
                });
                num = "".to_owned();
            }
            if let Some(mut tail) = stack.pop() {
                tail.push(List { values: current });
                current = tail;
            }
        }
    }

    if let List { values } = &current[0] {
        values.clone()
    } else {
        panic!("Top should be list")
    }
}

pub fn parse_pairs(input: &Vec<String>) -> Vec<Packet> {
    let mut packets: Vec<Packet> = Vec::new();
    for group in input.chunks(3) {
        if let [first, second, _] = group {
            packets.push(Packet {
                left: parse_list(first),
                right: parse_list(second),
            });
        } else if let [first, second] = group {
            packets.push(Packet {
                left: parse_list(first),
                right: parse_list(second),
            });
        }
    }

    packets
}

#[test]
pub fn test_list() {
    let line = "[9,3,3,10,7]";
    let data = parse_list(line);
    println!("Data: {:?}", data);

    let line = "[[1],[2,3,4]]";

    let data = parse_list(line);
    println!("Data: {:?}", data);
    assert!(data.len() == 2);
}
