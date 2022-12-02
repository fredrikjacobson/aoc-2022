use std::str::FromStr;

use crate::helper::{read_lines, ParseError};

#[derive(Clone, Copy, Debug)]
enum ScissorHand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for ScissorHand {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" | "A" => Ok(ScissorHand::Rock),
            "Y" | "B" => Ok(ScissorHand::Paper),
            "Z" | "C" => Ok(ScissorHand::Scissors),
            _ => Err(ParseError::new("Not part of enum")),
        }
    }

    type Err = ParseError;
}

impl From<u8> for ScissorHand {
    fn from(val: u8) -> Self {
        match val {
            1 => ScissorHand::Rock,
            2 => ScissorHand::Paper,
            3 => ScissorHand::Scissors,
            0 => ScissorHand::Scissors,
            _ => panic!("Invalid value"),
        }
    }
}

impl FromStr for Outcome {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(ParseError::new("Not part of enum")),
        }
    }

    type Err = ParseError;
}

#[test]
pub fn test_day2_pt_1() {
    let strategy: Vec<String> = read_lines(2, false);
    let plays: Vec<(ScissorHand, ScissorHand)> = parse_strategy(strategy, part1_strategy);

    let score: u32 = plays.iter().map(calculate_score).sum();

    println!("Part1 score is {:?}", score);
    assert!(false);
}

#[test]
pub fn test_day2_pt_2() {
    let strategy: Vec<String> = read_lines(2, false);
    let plays: Vec<(ScissorHand, ScissorHand)> = parse_strategy(strategy, part2_strategy);

    let score: u32 = plays.iter().map(calculate_score).sum();

    println!("Part1 score is {:?}", score);
    assert!(false);
}

fn parse_strategy(
    strategy: Vec<String>,
    choose_hand: fn(ScissorHand, &str) -> ScissorHand,
) -> Vec<(ScissorHand, ScissorHand)> {
    strategy
        .iter()
        .map(|s| {
            let inputs: Vec<&str> = s.split(" ").collect();
            let other = get_strategy(inputs[0]);
            let suggestion = choose_hand(other, inputs[1]);
            (other, suggestion)
        })
        .collect()
}

fn part2_strategy(other: ScissorHand, hand: &str) -> ScissorHand {
    match hand.parse::<Outcome>() {
        Ok(Outcome::Win) => ((other as u8 + 1) % 3).into(),
        Ok(Outcome::Draw) => other,
        Ok(Outcome::Loose) => ((other as i8 - 1).abs() as u8).into(),
        Err(_) => panic!("Could not parse hand"),
    }
}

fn part1_strategy(other: ScissorHand, hand: &str) -> ScissorHand {
    hand.parse::<ScissorHand>().unwrap()
}

fn get_strategy(hand: &str) -> ScissorHand {
    hand.parse::<ScissorHand>().unwrap()
}

fn calculate_score(hands: &(ScissorHand, ScissorHand)) -> u32 {
    let (other, our) = hands;

    let win_points = match (other, our) {
        (ScissorHand::Rock, ScissorHand::Paper)
        | (ScissorHand::Paper, ScissorHand::Scissors)
        | (ScissorHand::Scissors, ScissorHand::Rock) => 6,
        (ScissorHand::Paper, ScissorHand::Paper)
        | (ScissorHand::Rock, ScissorHand::Rock)
        | (ScissorHand::Scissors, ScissorHand::Scissors) => 3,
        (ScissorHand::Paper, ScissorHand::Rock)
        | (ScissorHand::Rock, ScissorHand::Scissors)
        | (ScissorHand::Scissors, ScissorHand::Paper) => 0,
    };

    win_points + *our as u8 as u32
}
