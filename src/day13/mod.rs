use packet::PacketData::*;

use packet::{Packet, PacketData};

use crate::helper::read_string;

mod packet;

#[test]
pub fn test_day13_pt_1() {
    let input: String = read_string(13, false);
    let pairs = packet::parse_packets(&input);

    let mut sum = 0;
    for (i, pair) in pairs.iter().enumerate() {
        let is_right_order = compare(&pair.left, &pair.right);
        if let Some(true) = is_right_order {
            // print_pair(pair);
            sum = sum + i + 1;
        }
        // println!("Pair {}  - {:?}", i + 1, is_right_order)
    }
    println!("{}", pairs.len());
    println!("Part1 score is {:?}", sum);
    assert_eq!(sum, 5682);
}

fn print_pair(pair: &Packet) {
    let mut line = "".to_owned();
    line.push('[');
    for item in &pair.left {
        line.push_str(&print_data(item));
        line.push(',')
    }
    line.push(']');

    line.push_str(" vs ");

    line.push('[');
    for item in &pair.right {
        line.push_str(&print_data(item));
        line.push(',')
    }
    line.push(']');

    println!("{}", line);
}

fn print_data(data: &PacketData) -> String {
    let mut data_str = "".to_owned();
    match data {
        Integer { value } => data_str.push_str(&value.to_string()),
        List { values } => {
            data_str.push('[');
            for val in values {
                data_str.push_str(&print_data(val));
                data_str.push(',')
            }
            data_str.push(']');
        }
    }

    data_str
}

pub fn compare(left: &Vec<PacketData>, right: &Vec<PacketData>) -> Option<bool> {
    // println!("Compare {:?} vs {:?}", left, right);
    for i in 0..left.len().max(right.len()) {
        match (left.get(i), right.get(i)) {
            (None, Some(_)) => return Some(true),
            (Some(_), None) => return Some(false),
            (Some(Integer { value: left_value }), Some(Integer { value: right_value })) => {
                if left_value < right_value {
                    // println!("Left side is smaller {} vs {}", left_value, right_value);
                    return Some(true);
                } else if right_value < left_value {
                    // println!("Right side is smaller {} vs {}", left_value, right_value);
                    return Some(false);
                }

                ()
            }
            (
                Some(List { values: left_value }),
                Some(List {
                    values: right_value,
                }),
            ) => {
                // println!("Comparing {:?} vs {:?}", left_value, right_value);
                match compare(left_value, right_value) {
                    Some(true) => return Some(true),
                    Some(false) => return Some(false),
                    None => (),
                }
            }

            (Some(List { values: left_value }), Some(Integer { value: right_value })) => {
                match compare(
                    left_value,
                    &vec![Integer {
                        value: *right_value,
                    }],
                ) {
                    Some(true) => return Some(true),
                    Some(false) => return Some(false),
                    None => (),
                };
            }
            (
                Some(Integer { value: left_value }),
                Some(List {
                    values: right_value,
                }),
            ) => {
                // println!(
                //     "Lifting left value {:?} to match {:?}",
                //     left_value, right_value
                // );
                match compare(&vec![Integer { value: *left_value }], right_value) {
                    Some(true) => return Some(true),
                    Some(false) => {
                        return Some(false);
                    }
                    None => (),
                };
            }
            (None, None) => (),
        };
    }

    return None;
}

#[test]
pub fn test_day13_pt_2() {
    let input: String = read_string(13, false);
    let mut pairs: Vec<PacketData> = flatten_packets(input);
    let decoder_packets = [
        List {
            values: vec![List {
                values: vec![Integer { value: 2 }],
            }],
        },
        List {
            values: vec![List {
                values: vec![Integer { value: 6 }],
            }],
        },
    ];

    pairs.push(decoder_packets[0].clone());
    pairs.push(decoder_packets[1].clone());

    pairs.sort();

    let decoders: Vec<usize> = find_decoder_packets(&pairs);
    for pair in pairs {
        println!("{}", &print_data(&pair));
    }
    println!("{:?}", &decoders);
    let score = (decoders[0] + 1) * (decoders[1] + 1);
    println!("Part2 score is {:?}", score);
    assert_eq!(score, 20304);
}

fn find_decoder_packets(pairs: &Vec<PacketData>) -> Vec<usize> {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, p)| match p {
            List { values } if values.len() == 1 => match values.get(0) {
                Some(List { values }) if values.len() == 1 => match values.get(0) {
                    Some(Integer { value: 6 }) => Some(i),
                    Some(Integer { value: 2 }) => Some(i),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
        .collect()
}

fn flatten_packets(input: String) -> Vec<PacketData> {
    packet::parse_packets(&input)
        .iter()
        .flat_map(|pair| {
            vec![
                List {
                    values: pair.left.clone(),
                },
                List {
                    values: pair.right.clone(),
                },
            ]
        })
        .collect()
}
