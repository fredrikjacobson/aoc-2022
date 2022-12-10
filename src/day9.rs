use anyhow::{anyhow, Context, Result};
use regex::internal::Inst;
use std::{collections::HashSet, hash::Hash, str::FromStr};

use crate::helper::{read_lines, ParseError};

#[derive(Debug, Clone)]
enum Command {
    Up { steps: usize },
    Right { steps: usize },
    Down { steps: usize },
    Left { steps: usize },
}

#[derive(Debug, Clone)]
enum Instruction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cmd, step)) = s.split_once(" ") {
            let steps = step.parse::<u8>()?;
            match cmd {
                "U" => Ok(Command::Up {
                    steps: steps.into(),
                }),
                "R" => Ok(Command::Right {
                    steps: steps.into(),
                }),
                "D" => Ok(Command::Down {
                    steps: steps.into(),
                }),
                "L" => Ok(Command::Left {
                    steps: steps.into(),
                }),
                _ => Err(anyhow!("Missing attribute: {}", cmd)),
            }
        } else {
            Err(anyhow!("Not a command"))
        }
    }
}

#[test]
pub fn test_day9_pt_1() {
    let lines: Vec<String> = read_lines(9, false);
    let commands = lines.iter().map(|l| l.parse::<Command>().unwrap());
    let instructions: Vec<Instruction> = commands.flat_map(to_instruction).collect();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::<(i16, i16)>::new();

    for instruction in &instructions {
        head = move_head(head, instruction);
        tail = move_tail(&tail, head);

        // println!("head: {:?} tail: {:?}", head, tail);
        visited.insert(tail);
    }
    println!("Part1 score is {:?}", visited.len());
    assert!(false);
}

fn move_head(position: (i16, i16), instruction: &Instruction) -> (i16, i16) {
    let (x, y) = position;
    match instruction {
        Instruction::Up => (x, y + 1),
        Instruction::Right => (x + 1, y),
        Instruction::Down => (x, y - 1),
        Instruction::Left => (x - 1, y),
    }
}

fn move_tail(tail: &(i16, i16), head: (i16, i16)) -> (i16, i16) {
    let (tail_x, tail_y) = tail.clone();
    let (head_x, head_y) = head;

    if tail_x == head_x && tail_y.abs_diff(head_y) > 1 {
        if tail_y > head_y {
            (tail_x, tail_y - 1)
        } else {
            (tail_x, tail_y + 1)
        }
    } else if tail_y == head_y && tail_x.abs_diff(head_x) > 1 {
        if tail_x > head_x {
            (tail_x - 1, tail_y)
        } else {
            (tail_x + 1, tail_y)
        }
    } else if head_x > tail_x && head_y > tail_y && not_close_diagonal(head, tail) {
        // Diagonally top-right
        (tail_x + 1, tail_y + 1)
    } else if head_x > tail_x && head_y < tail_y && not_close_diagonal(head, tail) {
        // Diagonally bottom-right
        (tail_x + 1, tail_y - 1)
    } else if head_x < tail_x && head_y > tail_y && not_close_diagonal(head, tail) {
        // Diagonally top-left
        (tail_x - 1, tail_y + 1)
    } else if head_x < tail_x && head_y < tail_y && not_close_diagonal(head, tail) {
        // Diagonally bottom-left
        (tail_x - 1, tail_y - 1)
    } else {
        tail.clone()
    }
}
fn not_close_diagonal(head: (i16, i16), tail: &(i16, i16)) -> bool {
    head.1.abs_diff(tail.1) + head.0.abs_diff(tail.0) > 2
}
fn to_instruction(cmd: Command) -> Vec<Instruction> {
    match cmd {
        Command::Up { steps } => vec![Instruction::Up; steps],
        Command::Right { steps } => vec![Instruction::Right; steps],
        Command::Down { steps } => vec![Instruction::Down; steps],
        Command::Left { steps } => vec![Instruction::Left; steps],
    }
}

#[test]
pub fn test_day9_pt_2() {
    let lines: Vec<String> = read_lines(9, false);
    let commands = lines.iter().map(|l| l.parse::<Command>().unwrap());
    let instructions: Vec<Instruction> = commands.flat_map(to_instruction).collect();

    let mut head = (0, 0);
    let mut tails = [(0, 0); 9];
    let mut visited = HashSet::<(i16, i16)>::new();

    for instruction in &instructions {
        head = move_head(head, instruction);
        let mut next_knot = head;
        for mut tail in tails.iter_mut() {
            let new_tail = move_tail(tail, next_knot);
            tail.0 = new_tail.0;
            tail.1 = new_tail.1;

            next_knot = new_tail;
        }

        // println!("head: {:?} tail: {:?}", head, tail);
        visited.insert(tails[8]);
    }
    println!("Part2 score is {:?}", visited.len());
    assert!(false);
}
