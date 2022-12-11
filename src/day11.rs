use std::{collections::HashSet, hash::Hash, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while},
    character::{
        complete::{self, digit1, newline, one_of},
        is_alphanumeric,
        streaming::char,
    },
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, tuple},
    IResult, Parser,
};
use num::{self, integer};

use crate::helper::{read_lines, read_string, ParseError};

#[test]
pub fn test_parse_monkey() {
    let monkey_input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"#;

    let (_, result) = parse_monkey(monkey_input).unwrap();
    assert_eq!(result.starting_items, vec![79, 98]);
    assert_eq!(
        result.operation,
        Operation {
            lhs: Value::Old,
            op: Arithmetic::Multiply,
            rhs: Value::Value { x: 19 }
        }
    );
}
#[derive(PartialEq, Debug)]
struct Monkey {
    starting_items: Vec<u64>,
    operation: Operation,
    divisible_test: u64,
    monkey_true: u64,
    monkey_false: u64,
    inspected: u64,
}
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, (starting_items, operation, divisible_test, monkey_true, monkey_false)) =
        preceded(
            pair(take_while(|c| c != '\n'), newline),
            tuple((
                delimited(
                    tag("  Starting items: "),
                    separated_list1(tag(", "), map_res(digit1, |s: &str| s.parse::<u64>())),
                    newline,
                ),
                delimited(tag("  Operation: new = "), parse_operation, newline),
                delimited(tag("  Test: divisible by "), complete::u64, newline),
                delimited(tag("    If true: throw to monkey "), complete::u64, newline),
                preceded(tag("    If false: throw to monkey "), complete::u64),
            )),
        )(input)?;

    Ok((
        input,
        Monkey {
            starting_items,
            operation,
            divisible_test,
            monkey_true,
            monkey_false,
            inspected: 0,
        },
    ))
}

#[derive(PartialEq, Debug)]
enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(PartialEq, Debug)]
enum Value {
    Value { x: i32 },
    Old,
}
#[derive(PartialEq, Debug)]
struct Operation {
    lhs: Value,
    op: Arithmetic,
    rhs: Value,
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (lhs, op, rhs)) = tuple((
        parse_operand,
        delimited(
            tag(" "),
            one_of("+-*/").map(|op| match op {
                '+' => Arithmetic::Add,
                '-' => Arithmetic::Subtract,
                '*' => Arithmetic::Multiply,
                '/' => Arithmetic::Divide,
                _ => panic!("Could not parse token"),
            }),
            tag(" "),
        ),
        parse_operand,
    ))(input)?;

    Ok((input, Operation { lhs, op, rhs }))
}

fn parse_operand(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        complete::i32.map(|num: i32| Value::Value { x: num }),
    ))(input)
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), parse_monkey)(input)
}

#[test]
pub fn test_day11_pt_1() {
    let input: String = read_string(11, false);
    let (_, mut monkeys) = parse_monkeys(&input).unwrap();

    let size = monkeys.len();
    for _ in 0..20 {
        for i in 0..size {
            let throws = monkey_inspect(&monkeys[i], &|num| num / 3);
            monkeys[i].starting_items = Vec::new();
            monkeys[i].inspected = monkeys[i].inspected + throws.len() as u64;

            for (destination, item) in throws {
                monkeys[destination as usize].starting_items.push(item);
            }
        }
    }

    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();
    inspected.sort_by(|a, b| b.cmp(a));
    let monkey_business = inspected[0] * inspected[1];
    println!("Part1 score is {:?}", monkey_business);
    assert_eq!(monkey_business, 51075);
}

type WorryManager = dyn Fn(u64) -> u64;
fn monkey_inspect(monkey: &Monkey, worry_manager: &WorryManager) -> Vec<(u64, u64)> {
    let mut throws = Vec::new();
    for item in monkey.starting_items.iter() {
        let new_worry = get_new_worry_level(*item, &monkey.operation, worry_manager);
        if new_worry % monkey.divisible_test == 0 {
            throws.push((monkey.monkey_true, new_worry));
        } else {
            throws.push((monkey.monkey_false, new_worry));
        }
    }
    throws
}

fn get_new_worry_level(item: u64, operation: &Operation, worry_manager: &WorryManager) -> u64 {
    let new_worry = match operation.op {
        Arithmetic::Add => to_value(item, &operation.lhs) + to_value(item, &operation.rhs),
        Arithmetic::Subtract => to_value(item, &operation.lhs) - to_value(item, &operation.rhs),
        Arithmetic::Multiply => to_value(item, &operation.lhs) * to_value(item, &operation.rhs),
        Arithmetic::Divide => to_value(item, &operation.lhs) / to_value(item, &operation.rhs),
    };

    worry_manager(new_worry)
}

fn to_value(current: u64, value: &Value) -> u64 {
    match value {
        Value::Old => current,
        Value::Value { x } => *x as u64,
    }
}

#[test]
pub fn test_day11_pt_2() {
    let input: String = read_string(11, false);
    let (_, mut monkeys) = parse_monkeys(&input).unwrap();

    let lcm = manage_worry(&monkeys.iter().map(|m| m.divisible_test).collect()) as u64;

    let worry_manager_factory = |modulus: u64| move |num: u64| num % modulus;

    println!("Worries LCM: {}", lcm);

    let size = monkeys.len();
    for _ in 0..10000 {
        for i in 0..size {
            let throws = monkey_inspect(&monkeys[i], &worry_manager_factory(lcm));
            monkeys[i].starting_items = Vec::new();
            monkeys[i].inspected = monkeys[i].inspected + throws.len() as u64;

            for (destination, item) in throws {
                monkeys[destination as usize].starting_items.push(item);
            }
        }
    }
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {}: {:?}", i + 1, monkey);
    }

    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();
    inspected.sort_by(|a, b| b.cmp(a));
    let monkey_business = inspected[0] as u64 * inspected[1] as u64;
    println!("Part2 score is {:?}", monkey_business);
    assert!(false);
}

fn manage_worry(worries: &Vec<u64>) -> u128 {
    let res = worries
        .iter()
        .map(|num| *num as u128)
        .reduce(|start: u128, elem| integer::lcm(start.into(), elem.into()))
        .unwrap();
    res
}
