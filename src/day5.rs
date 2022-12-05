use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use regex::Regex;

use crate::helper::{read_lines, ParseError};

#[derive(Debug)]
struct Crates {
    pub stacks: HashMap<u8, Vec<char>>,
}

#[test]
pub fn test_day5_pt_1() {
    let lines: Vec<String> = read_lines(5, false);
    let mut placement: Vec<&String> = lines.iter().take_while(|line| !line.is_empty()).collect();
    placement.reverse();
    dbg!(&placement);
    let movements = lines.iter().skip(placement.len() + 1);

    let mut crates = parse_placement_into_crates(placement);

    for movement in movements {
        let movement = movement.parse::<Movement>().unwrap();

        (0..movement.num_containers).for_each(|_| {
            let from_stack = crates.stacks.get_mut(&movement.from).unwrap();
            let container = from_stack.pop().unwrap();
            let to_stack = crates.stacks.get_mut(&movement.to).unwrap();
            to_stack.push(container);
        });
    }
    let top = get_top_container(crates);
    println!("Part1 score is {:?}", &top);
    assert!(false);
}

fn parse_placement_into_crates(placement: Vec<&String>) -> Crates {
    let mut crates = Crates {
        stacks: HashMap::new(),
    };
    let line_len = placement[0].len();
    let num_stacks = placement[0]
        .chars()
        .nth(line_len - 2)
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;
    for line in placement.iter().skip(1) {
        let chars: Vec<char> = line.chars().collect();
        for stack in 1..num_stacks + 1 {
            let container = chars[(stack - 1) * 4 + 1];
            if container.is_whitespace() {
                continue;
            }
            if let Some(entry) = crates.stacks.get_mut(&(stack as u8)) {
                entry.push(container);
            } else {
                crates.stacks.insert(stack as u8, vec![container]);
            }
        }
    }
    crates
}

#[test]
pub fn test_day5_pt_2() {
    let lines: Vec<String> = read_lines(5, false);
    let mut placement: Vec<&String> = lines.iter().take_while(|line| !line.is_empty()).collect();
    placement.reverse();
    let movements = lines.iter().skip(placement.len() + 1);

    let mut crates = parse_placement_into_crates(placement);

    for movement in movements {
        let movement = movement.parse::<Movement>().unwrap();

        let mut temp_stack: Vec<char> = Vec::new();
        (0..movement.num_containers).for_each(|_| {
            let from_stack = crates.stacks.get_mut(&movement.from).unwrap();
            let container = from_stack.pop().unwrap();
            temp_stack.push(container);
        });

        for moved_container in temp_stack.iter().rev() {
            let to_stack = crates.stacks.get_mut(&movement.to).unwrap();
            to_stack.push(*moved_container);
        }
    }
    let top = get_top_container(crates);
    println!("Part2 score is {:?}", top);
    assert!(false);
}

fn get_top_container(crates: Crates) -> String {
    let mut top = "".to_string();
    for i in 0..crates.stacks.len() {
        let top_container = crates
            .stacks
            .get(&((i + 1) as u8))
            .unwrap()
            .last()
            .unwrap()
            .to_string();
        top.push_str(&top_container);
    }
    top
}

struct Movement {
    from: u8,
    to: u8,
    num_containers: u8,
}

impl FromStr for Movement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_move(s))
    }
}
fn parse_move(movement: &str) -> Movement {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let matches = re.captures(movement).unwrap();

    let from = matches.get(2).unwrap().as_str().parse::<u8>().unwrap();
    let to = matches.get(3).unwrap().as_str().parse::<u8>().unwrap();
    let num_containers = matches.get(1).unwrap().as_str().parse::<u8>().unwrap();
    Movement {
        from,
        to,
        num_containers,
    }
}
#[test]
pub fn test_movement() {
    let movement_str = "move 1 from 2 to 1".to_owned();
    let movement = parse_move(&movement_str);
    assert_eq!(movement.from, 2);
    assert_eq!(movement.to, 1);
    assert_eq!(movement.num_containers, 1);
}
