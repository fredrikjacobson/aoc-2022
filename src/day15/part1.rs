use std::collections::HashSet;

use crate::{
    day15::sensor::{parse_sensor, Sensor},
    helper::read_lines,
};

// #[test]
pub fn test_day15_pt_1() {
    let lines: Vec<String> = read_lines(15, false);
    let sensors: Vec<Sensor> = lines
        .iter()
        .map(|line| parse_sensor(line).unwrap().1)
        .collect();
    let size = setup_grid(&sensors);
    println!("Part1 score is {:?}", size);
    assert!(false);
}

enum Cell {
    Cover,
    Empty,
}

fn setup_grid(sensors: &Vec<Sensor>) -> usize {
    let search_row = 2_000_000;
    let mut xs = HashSet::new();
    for sensor in sensors {
        let manhattan_distance = sensor.manhattan_distance;
        if sensor.position.y <= search_row && sensor.position.y + manhattan_distance >= search_row {
            let mut y = sensor.position.y + manhattan_distance;
            let mut x_distance = 0;
            while y >= search_row {
                xs.insert(sensor.position.x + x_distance);
                xs.insert(sensor.position.x - x_distance);
                y = y - 1;
                x_distance = x_distance + 1;
            }
        } else if sensor.position.y >= search_row
            && sensor.position.y - manhattan_distance <= search_row
        {
            let mut y = sensor.position.y - manhattan_distance;
            let mut x_distance = 0;
            while y <= search_row {
                xs.insert(sensor.position.x + x_distance);
                xs.insert(sensor.position.x - x_distance);
                y = y + 1;
                x_distance = x_distance + 1;
            }
        }
    }

    for sensor in sensors {
        if (sensor.position.y == search_row) {
            xs.remove(&sensor.position.x);
        } else if (sensor.closest_beacon.y == search_row) {
            xs.remove(&sensor.closest_beacon.x);
        }
    }

    xs.len()
}
