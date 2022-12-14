use std::{cmp::Ordering, collections::HashSet, error::Error, hash::Hash, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{self, digit1, newline, one_of},
    combinator::map_res,
    error::{ErrorKind, ParseError},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
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

pub fn parse_packets(input: &str) -> Vec<Packet> {
    let (_, packets) = separated_list1(
        newline,
        tuple((new_parse, new_parse)).map(|(first, second)| Packet {
            left: if let List { values } = first {
                values
            } else {
                panic!("Should be list")
            },
            right: if let List { values } = second {
                values
            } else {
                panic!("Should be list")
            },
        }),
    )(input)
    .unwrap();

    packets
}

fn new_parse(input: &str) -> IResult<&str, PacketData> {
    let mut current: Vec<PacketData> = Vec::new();
    let mut rest = input.clone();
    loop {
        if rest.is_empty() {
            break;
        }

        if let Ok((tail, result)) = alt((
            delimited(tag("["), new_parse, tag("]")),
            complete::u32.map(|num| Integer {
                value: num as usize,
            }),
        ))(rest)
        {
            current.push(result);
            rest = tail;
        }

        if let Ok((new_tail, _)) = tag::<_, _, (_, ErrorKind)>(",")(rest) {
            rest = new_tail;
        } else if let Ok((new_tail, _)) = newline::<_, (_, ErrorKind)>(rest) {
            rest = new_tail;
            break;
        } else {
            break;
        }
    }

    Ok((rest, List { values: current }))
}

#[test]
pub fn test_new_parse() {
    let (input, data) = new_parse("[1,2,3,4,5]").unwrap();
    let (input, data) = new_parse("[1,[[2]],3,4,5]").unwrap();
    println!("{:?}", data);
    assert!(false)
}

#[test]
pub fn test_list() {
    let line = "[9,3,3,10,7]";
    let (_, data) = new_parse(line).unwrap();
    println!("Data: {:?}", data);

    let line = "[[1],[2,3,4]]";

    let (_, data) = new_parse(line).unwrap();
    println!("Data: {:?}", data);
}
