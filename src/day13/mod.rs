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

use crate::helper::{read_lines, ParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketData {
    Integer { value: usize },
    List { values: Vec<PacketData> },
}
use PacketData::*;
#[derive(Debug, Clone)]
struct Packet {
    left: Vec<PacketData>,
    right: Vec<PacketData>,
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

fn parse_pairs(input: &Vec<String>) -> Vec<Packet> {
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

#[test]
pub fn test_day13_pt_1() {
    let lines: Vec<String> = read_lines(13, false);
    let pairs = parse_pairs(&lines);

    let mut sum = 0;
    for (i, pair) in pairs.iter().enumerate() {
        let is_right_order = compare(&pair.left, &pair.right);
        if let Some(true) = is_right_order {
            print_pair(pair);
            sum = sum + i + 1;
        }
        // println!("Pair {}  - {:?}", i + 1, is_right_order)
    }
    println!("{}", pairs.len());
    println!("Part1 score is {:?}", sum);
    assert!(false);
}

fn print_pair(pair: &Packet) {
    let mut line = "".to_owned();
    line.push('[');
    for item in &pair.left {
        line.push_str(&print_data(item));
        line.push(',')
    }
    line.push(']');

    line.push_str(" vs ");

    line.push('[');
    for item in &pair.right {
        line.push_str(&print_data(item));
        line.push(',')
    }
    line.push(']');

    println!("{}", line);
}

fn print_data(data: &PacketData) -> String {
    let mut data_str = "".to_owned();
    match data {
        Integer { value } => data_str.push_str(&value.to_string()),
        List { values } => {
            data_str.push('[');
            for val in values {
                data_str.push_str(&print_data(val));
                data_str.push(',')
            }
            data_str.push(']');
        }
    }

    data_str
}

fn compare(left: &Vec<PacketData>, right: &Vec<PacketData>) -> Option<bool> {
    // println!("Compare {:?} vs {:?}", left, right);
    for i in 0..left.len().max(right.len()) {
        match (left.get(i), right.get(i)) {
            (None, Some(_)) => return Some(true),
            (Some(_), None) => return Some(false),
            (Some(Integer { value: left_value }), Some(Integer { value: right_value })) => {
                if left_value < right_value {
                    // println!("Left side is smaller {} vs {}", left_value, right_value);
                    return Some(true);
                } else if right_value < left_value {
                    // println!("Right side is smaller {} vs {}", left_value, right_value);
                    return Some(false);
                }

                ()
            }
            (
                Some(List { values: left_value }),
                Some(List {
                    values: right_value,
                }),
            ) => {
                // println!("Comparing {:?} vs {:?}", left_value, right_value);
                match compare(left_value, right_value) {
                    Some(true) => return Some(true),
                    Some(false) => return Some(false),
                    None => (),
                }
            }

            (Some(List { values: left_value }), Some(Integer { value: right_value })) => {
                match compare(
                    left_value,
                    &vec![Integer {
                        value: *right_value,
                    }],
                ) {
                    Some(true) => return Some(true),
                    Some(false) => return Some(false),
                    None => (),
                };
            }
            (
                Some(Integer { value: left_value }),
                Some(List {
                    values: right_value,
                }),
            ) => {
                // println!(
                //     "Lifting left value {:?} to match {:?}",
                //     left_value, right_value
                // );
                match compare(&vec![Integer { value: *left_value }], right_value) {
                    Some(true) => return Some(true),
                    Some(false) => {
                        return Some(false);
                    }
                    None => (),
                };
            }
            (None, None) => (),
        };
    }

    return None;
}

#[test]
pub fn test_day13_pt_2() {
    let lines: Vec<String> = read_lines(13, false);
    let mut pairs: Vec<PacketData> = parse_pairs(&lines)
        .iter()
        .flat_map(|pair| {
            vec![
                List {
                    values: pair.left.clone(),
                },
                List {
                    values: pair.right.clone(),
                },
            ]
        })
        .collect();

    pairs.push(List {
        values: vec![List {
            values: vec![Integer { value: 2 }],
        }],
    });
    pairs.push(List {
        values: vec![List {
            values: vec![Integer { value: 6 }],
        }],
    });

    pairs.sort();

    let decoders: Vec<usize> = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, p)| match p {
            List { values } if values.len() == 1 => match values.get(0) {
                Some(List { values }) if values.len() == 1 => match values.get(0) {
                    Some(Integer { value: 6 }) => Some(i),
                    Some(Integer { value: 2 }) => Some(i),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
        .collect();
    for pair in pairs {
        println!("{}", &print_data(&pair));
    }
    println!("{:?}", &decoders);
    println!("Part2 score is {:?}", (decoders[0] + 1) * (decoders[1] + 1));
    assert!(false);
}
