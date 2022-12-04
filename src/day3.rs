use std::{collections::HashSet, str::FromStr};

use crate::helper::{read_lines, ParseError};

#[test]
pub fn ascii_code() {
    assert_eq!(to_ascii_value(&'p'), 16);
}
#[test]
pub fn test_day3_pt_1() {
    let rucksacks: Vec<String> = read_lines(3, false);
    let common_items: Vec<char> = rucksacks
        .iter()
        .map(|row| {
            let (first, second) = row.split_at(row.len() / 2);
            let first_set: HashSet<char> = HashSet::from_iter(first.chars());
            let second_set: HashSet<char> = HashSet::from_iter(second.chars());

            let common: Vec<&char> = first_set.intersection(&second_set).collect();
            assert_eq!(common.len(), 1);
            return common[0].clone();
        })
        .collect();

    let scores: Vec<u32> = common_items.iter().map(to_ascii_value).collect();

    println!("Part1 score is {:?}", scores.iter().sum::<u32>());
    assert!(false);
}

struct ElveGroup {
    items: Vec<String>,
    curr: usize,
    next: (String, String, String),
}

impl ElveGroup {
    fn new(items: Vec<String>) -> ElveGroup {
        ElveGroup {
            items,
            curr: 0,
            next: ("".to_owned(), "".to_owned(), "".to_owned()),
        }
    }
}
// Implement `Iterator` for `ElveGroup`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for ElveGroup {
    // We can refer to this type using Self::Item
    type Item = (String, String, String);

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.curr;

        if current_index < self.items.len() {
            self.next = (
                self.items[current_index].clone(),
                self.items[current_index + 1].clone(),
                self.items[current_index + 2].clone(),
            );
            self.curr = current_index + 3;
            Some(self.next.clone())
        } else {
            None
        }
    }
}

#[test]
pub fn test_day3_pt_2() {
    let rucksacks: Vec<String> = read_lines(3, false);
    let elve_groups = ElveGroup::new(rucksacks);
    let common_items: Vec<char> = elve_groups
        .map(|(first, second, third)| {
            let first_set: HashSet<char> = HashSet::from_iter(first.chars());
            let second_set: HashSet<char> = HashSet::from_iter(second.chars());
            let third_set: HashSet<char> = HashSet::from_iter(third.chars());

            let common_set: HashSet<char> =
                HashSet::from_iter(first_set.intersection(&second_set).copied());
            let common_set = common_set.intersection(&third_set);

            let common: Vec<&char> = common_set.collect();
            println!("Common items {:?}", common);
            assert_eq!(common.len(), 1);
            return common[0].clone();
        })
        .collect();

    let scores: Vec<u32> = common_items.iter().map(to_ascii_value).collect();

    println!("Part1 score is {:?}", scores.iter().sum::<u32>());
    assert!(false);
}

fn to_ascii_value(c: &char) -> u32 {
    if c.is_ascii_uppercase() {
        *c as u32 - 'A' as u32 + 27
    } else {
        *c as u32 - 'a' as u32 + 1
    }
}
