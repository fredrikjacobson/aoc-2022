use std::cmp;

use crate::helper::read_lines;

#[test]
pub fn test_day1_pt_1() {
    let mut elves: Vec<usize> = Vec::new();
    let report: Vec<String> = read_lines(1, false);

    let mut sum = 0;
    for calorie in report {
        if calorie == "" {
            elves.push(sum);
            sum = 0;
        } else {
            sum = sum + calorie.parse::<usize>().unwrap();
        }
    }
    if sum > 0 {
        elves.push(sum);
    }

    let max_value = elves.iter().fold(0, |current_max, elem| {
        cmp::max(current_max, elem.to_owned())
    });

    println!("Part1 Max number is {:?}", max_value);
    assert!(false);
}

#[test]
pub fn test_day1_pt_2() {
    let mut elves: Vec<usize> = Vec::new();
    let report: Vec<String> = read_lines(1, false);

    let mut sum = 0;
    for calorie in report {
        if calorie == "" {
            elves.push(sum);
            sum = 0;
        } else {
            sum = sum + calorie.parse::<usize>().unwrap();
        }
    }
    if sum > 0 {
        elves.push(sum);
    }

    let max_value: usize = elves
        .iter()
        .fold([&0, &0, &0], |current_max, elem| {
            if elem > current_max[0] {
                let mut new_max = [elem, &current_max[1], &current_max[2]];
                new_max.sort();
                new_max
            } else {
                current_max
            }
        })
        .into_iter()
        .sum();

    println!("Part2 Max number is {:?}", max_value);
    assert!(false);
}
