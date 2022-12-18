mod sensor;

use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    str::FromStr,
};

use crate::{
    day15::sensor::parse_sensor,
    helper::{read_lines, ParseError},
};

use self::sensor::{print_grid, Position, Sensor};

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
        let manhattan_distance = sensor.manhattan_distance();
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

fn search_grid(
    sensors: &Vec<Sensor>,
    lower: &Position,
    upper: &Position,
) -> Option<(usize, usize)> {
    let mut grid = vec![vec![true; (upper.x - lower.x) as usize]; (upper.y - lower.y) as usize];
    for sensor in sensors {
        if (sensor.within_bounds(lower, upper)) {
            for (row, range) in sensor.search_grid(lower.clone(), upper.clone()) {
                for x in range {
                    let adjusted_col = x - lower.x;
                    let adjusted_row = row - lower.y;
                    if (row >= lower.y && row < upper.y && x >= lower.x && x < upper.x) {
                        grid[(adjusted_row) as usize][(adjusted_col) as usize] = false;
                    }
                }
            }
        }
        // print_grid(&grid);
    }

    for (row_i, row) in grid.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col {
                return Some((col_i + lower.x as usize, row_i + lower.y as usize));
            }
        }
    }

    return None;
}

#[test]
pub fn test_day15_pt_2() {
    let answer = day15_pt_2();

    println!("Part2 score is {:?}", answer);
    assert_eq!(answer, (14, 11));
}

fn find_adjacent_sensors(sensors: &Vec<Sensor>) -> Position {
    let lower = &Position { x: 0, y: 0 };
    let upper = &Position {
        x: 4_000_000,
        y: 4_000_000,
    };
    for sensor_a in sensors {
        println!("Outer Sensor A {:?}", sensor_a.position);
        if sensor_a.within_bounds(&lower, &upper) {
            for sensor_b in sensors {
                if sensor_b.within_bounds(&lower, &upper) && sensor_a.intersects(sensor_b) {
                    for sensor_c in sensors {
                        if sensor_c.within_bounds(&lower, &upper)
                            && sensor_b.position != sensor_c.position
                            && sensor_a.position != sensor_c.position
                        {
                            for sensor_d in sensors {
                                println!("Inner Sensor D {:?}", sensor_d.position);
                                if sensor_d.within_bounds(&lower, &upper)
                                    && sensor_c.position != sensor_d.position
                                    && sensor_b.position != sensor_d.position
                                    && sensor_a.position != sensor_d.position
                                {
                                    let a: HashSet<Position> =
                                        HashSet::from_iter(sensor_a.line_points());
                                    let b: HashSet<Position> =
                                        HashSet::from_iter(sensor_b.line_points());
                                    let c: HashSet<Position> =
                                        HashSet::from_iter(sensor_c.line_points());
                                    let d: HashSet<Position> =
                                        HashSet::from_iter(sensor_d.line_points());

                                    let ab: HashSet<Position> =
                                        HashSet::from_iter(a.intersection(&b).copied());
                                    let abc = HashSet::from_iter(ab.intersection(&c).copied());
                                    let unique: Vec<Position> =
                                        abc.intersection(&d).copied().collect();

                                    if unique.len() == 1
                                        && !sensors.iter().any(|s| s.contains(&unique[0]))
                                    {
                                        // print_sensor_outline(
                                        //     &lower,
                                        //     &upper,
                                        //     &vec![&sensor_a, &sensor_b, &sensor_c, &sensor_d],
                                        // );
                                        println!("{:?}", unique);
                                        println!(
                                            "{:?}",
                                            vec![sensor_a, sensor_b, sensor_c, sensor_d]
                                        );

                                        return unique[0];
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Position { x: 0, y: 0 }
}

fn print_sensor_outline(lower: &Position, upper: &Position, sensors: &Vec<&Sensor>) {
    let mut grid = vec![vec![' '; 21]; 21];

    for sensor in sensors {
        for pos in sensor.line_points() {
            if pos.within(lower, upper) {
                grid[pos.y as usize][pos.x as usize] = '#';
            }
        }
    }

    for sensor in sensors {
        grid[sensor.position.y as usize][sensor.position.x as usize] = 'S';
    }
    for (i, line) in grid.iter().enumerate() {
        println!("Row: {: ^2} {}", i, line.iter().collect::<String>());
    }
}

pub fn day15_pt_2() -> (usize, usize) {
    let lines: Vec<String> = read_lines(15, false);
    let sensors: Vec<Sensor> = lines
        .iter()
        .map(|line| parse_sensor(line).unwrap().1)
        .collect();

    let sensor_match = find_adjacent_sensors(&sensors);
    println!("{:?}", sensor_match);
    return (0, 0);
    // if let Some(value) = naive_scan(sensors) {
    //     return value;
    // }
}

fn naive_scan(sensors: Vec<Sensor>) -> Option<(usize, usize)> {
    let max = 4_000_000;
    let partitions = 40;
    let bounds = max / partitions;
    let mut queue = VecDeque::new();
    for row in 0..partitions {
        for col in 0..partitions {
            queue.push_back((
                Position {
                    x: col * bounds,
                    y: row * bounds,
                },
                Position {
                    x: (col + 1) * bounds,
                    y: (row + 1) * bounds,
                },
            ));
        }
    }
    while let Some((lower, upper)) = queue.pop_back() {
        println!("Working on {:?} {:?}", lower, upper);
        if let Some(found) = search_grid(&sensors, &lower, &upper) {
            return Some(found);
        }
    }
    return Some((0, 0));
    None
}
