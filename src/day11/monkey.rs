use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{self, digit1, newline, one_of},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, tuple},
    IResult, Parser,
};

#[derive(PartialEq, Debug)]
pub enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(PartialEq, Debug)]
pub enum Value {
    Value { x: i32 },
    Old,
}
impl Value {
    fn as_option(&self) -> Option<u64> {
        match self {
            Value::Old => None,
            Value::Value { x } => Some(*x as u64),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Operation {
    pub lhs: Value,
    pub op: Arithmetic,
    pub rhs: Value,
}
impl Operation {
    fn evaluate(&self, item: u64) -> u64 {
        let lhs = self.lhs.as_option().unwrap_or(item);
        let rhs = self.rhs.as_option().unwrap_or(item);
        match self.op {
            Arithmetic::Add => lhs + rhs,
            Arithmetic::Subtract => lhs - rhs,
            Arithmetic::Multiply => lhs * rhs,
            Arithmetic::Divide => lhs / rhs,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Monkey {
    pub starting_items: Vec<u64>,
    pub operation: Operation,
    pub divisible_test: u64,
    pub monkey_true: u64,
    pub monkey_false: u64,
    pub inspected: u64,
}

type WorryManager = dyn Fn(u64) -> u64;

impl Monkey {
    pub fn inspect_and_throw(&mut self, worry_manager: &WorryManager) -> Vec<(u64, u64)> {
        let mut throws = Vec::new();
        for item in self.starting_items.iter() {
            let new_worry = self.operation.evaluate(*item);
            let new_worry = worry_manager(new_worry);
            if new_worry % self.divisible_test == 0 {
                throws.push((self.monkey_true, new_worry));
            } else {
                throws.push((self.monkey_false, new_worry));
            }
        }
        self.starting_items = Vec::new();
        self.inspected = self.inspected + throws.len() as u64;
        throws
    }
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

pub fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), parse_monkey)(input)
}

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
