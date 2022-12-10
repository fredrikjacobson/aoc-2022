use std::{collections::HashSet, hash::Hash, str::FromStr};

use crate::helper::{read_lines, ParseError};

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Add { x: i32 },
    NoOp,
}
struct CPU {
    cycle: usize,
    register: i32,
    instructions: Vec<Instruction>,
    current_instruction_index: usize,
    current_instruction: Option<Instruction>,
}

impl CPU {
    pub fn new(instructions: Vec<Instruction>) -> CPU {
        CPU {
            cycle: 0,
            register: 1,
            instructions,
            current_instruction_index: 0,
            current_instruction: None,
        }
    }
}

impl Iterator for CPU {
    // We can refer to this type using Self::Item
    type Item = i32;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Instruction::Add { x }) = self.current_instruction {
            self.current_instruction = None;
            self.cycle = self.cycle + 1;
            self.register = self.register + x;
            self.current_instruction_index = self.current_instruction_index + 1;
            // println!("Cycle: {} Register: {}", self.cycle, self.register - x);
            Some(self.register - x)
        } else if self.current_instruction_index < self.instructions.len() {
            match self.instructions[self.current_instruction_index] {
                Instruction::Add { x } => {
                    self.current_instruction =
                        Some(self.instructions[self.current_instruction_index].clone());
                    self.cycle = self.cycle + 1;
                    // println!("Cycle: {} Register: {}", self.cycle, self.register);
                    Some(self.register)
                }
                Instruction::NoOp => {
                    self.cycle = self.cycle + 1;
                    self.current_instruction_index = self.current_instruction_index + 1;
                    // println!("Cycle: {} Register: {}", self.cycle, self.register);
                    Some(self.register)
                }
            }
        } else {
            None
        }
    }
}

#[test]
pub fn test_day10_pt_1() {
    let lines: Vec<String> = read_lines(10, false);
    let mut cpu = parse_CPU(lines);

    let mut signals: Vec<(i32, i32)> = Vec::new();
    signals.push((20, cpu.nth(19).unwrap()));
    signals.push((60, cpu.nth(39).unwrap()));
    signals.push((100, cpu.nth(39).unwrap()));
    signals.push((140, cpu.nth(39).unwrap()));
    signals.push((180, cpu.nth(39).unwrap()));
    signals.push((220, cpu.nth(39).unwrap()));

    let sum = signals
        .iter()
        .fold(0, |acc, signal| acc + signal.0 * signal.1);
    println!("Part1 score is {:?} with sum {}", signals, sum);
    assert!(false);
}

#[test]
pub fn test_parse_instruction() {
    let input = r#"noop
	addx 3
	addx -5"#;
    let lines = input.split("\n").map(|l| l.to_owned()).collect();

    let expected = vec![
        Instruction::NoOp,
        Instruction::Add { x: 3 },
        Instruction::Add { x: -5 },
    ];

    let result = parse_CPU(lines);
    assert_eq!(expected, result.instructions);
}

#[test]
pub fn test_day10_pt_2() {
    let lines: Vec<String> = read_lines(10, false);
    let mut cpu = parse_CPU(lines);

    let mut crt = vec![vec![' '; 40]; 6];
    for row in 0..6 {
        for col in 0..40 {
            let register = cpu.next().unwrap();
            if (col as i32).abs_diff(register) < 2 {
                crt[row][col] = '#';
            } else {
                crt[row][col] = '.';
            }
        }
    }

    for row in 0..6 {
        println!("{}", String::from_iter(&crt[row]));
    }
    assert!(false);
}

fn parse_CPU(lines: Vec<String>) -> CPU {
    let instructions = lines
        .iter()
        .map(|i| {
            if i.starts_with("noop") {
                Instruction::NoOp
            } else {
                let (_, number) = i.split_once(" ").unwrap();
                Instruction::Add {
                    x: number.parse::<i32>().unwrap(),
                }
            }
        })
        .collect();
    let mut cpu = CPU::new(instructions);
    cpu
}
