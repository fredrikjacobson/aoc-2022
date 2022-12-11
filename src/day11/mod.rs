mod monkey;

use num::{self, integer};

use crate::{
    day11::monkey::{parse_monkeys, Arithmetic, Monkey, Operation, Value},
    helper::read_string,
};

#[test]
pub fn test_day11_pt_1() {
    let input: String = read_string(11, false);
    let (_, mut monkeys) = parse_monkeys(&input).unwrap();

    let size = monkeys.len();
    for _ in 0..20 {
        for i in 0..size {
            let throws = monkeys[i].inspect_and_throw(&|num| num / 3);

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

#[test]
pub fn test_day11_pt_2() {
    let input: String = read_string(11, false);
    let (_, mut monkeys) = parse_monkeys(&input).unwrap();

    let lcm = find_lcm(&monkeys.iter().map(|m| m.divisible_test).collect());

    println!("Worries LCM: {}", lcm);

    let size = monkeys.len();
    for _ in 0..10000 {
        for i in 0..size {
            let throws = monkeys[i].inspect_and_throw(&create_worry_manager(lcm));

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
    assert_eq!(monkey_business, 11741456163);
}

fn create_worry_manager(lcm: u64) -> impl Fn(u64) -> u64 {
    move |num: u64| num % lcm
}

fn find_lcm(worries: &Vec<u64>) -> u64 {
    let res = worries
        .iter()
        .map(|num| *num as u128)
        .reduce(|start: u128, elem| integer::lcm(start.into(), elem.into()))
        .unwrap();
    res as u64
}
