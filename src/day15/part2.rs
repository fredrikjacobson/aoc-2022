use crate::{
    day15::{
        find_adjacent_sensors,
        sensor::{parse_sensor, Sensor},
    },
    helper::read_lines,
};

use super::sensor::Position;

#[test]
pub fn test_day15_pt_2() {
    let answer = day15_pt_2();

    println!("Part2 score is {:?}", answer);
    let tuning_frequency = (answer.x as i64) * 4_000_000 + answer.y as i64;
    assert_eq!(tuning_frequency, 56_000_011);
}

pub fn day15_pt_2() -> Position {
    let lines: Vec<String> = read_lines(15, false);
    let sensors: Vec<Sensor> = lines
        .iter()
        .map(|line| parse_sensor(line).unwrap().1)
        .collect();

    let sensor_match = find_adjacent_sensors(&sensors, 4_000_000);
    println!("Match found {:?}", sensor_match);
    return sensor_match;
    // if let Some(value) = naive_scan(sensors) {
    //     return value;
    // }
}
