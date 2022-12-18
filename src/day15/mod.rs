mod line;
mod part1;
mod part2;
mod sensor;

use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    str::FromStr,
};

use crate::{
    day15::{line::Line, sensor::parse_sensor},
    helper::{read_lines, ParseError},
};

use self::sensor::{print_grid, Position, Sensor};

fn find_adjacent_sensors(sensors: &Vec<Sensor>, limit: i32) -> Position {
    let lower = &Position { x: 0, y: 0 };
    let upper = &Position { x: limit, y: limit };
    for sensor_a in sensors {
        println!("Outer Sensor A {:?}", sensor_a.position);
        if sensor_a.within_bounds(&lower, &upper) {
            for sensor_b in sensors {
                if sensor_b.within_bounds(&lower, &upper) {
                    if sensor_a.position != sensor_b.position
                        && Line::new(sensor_a.lines[0], sensor_a.x_range)
                            .perpendicular(&Line::new(sensor_b.lines[1], sensor_b.x_range))
                    {
                        println!("Lines match {:?} - {:?}", sensor_a, sensor_b);

                        for sensor_c in sensors {
                            for sensor_d in sensors {
                                if sensor_c.position != sensor_d.position
                                    && Line::new(sensor_c.lines[2], sensor_c.x_range).perpendicular(
                                        &Line::new(sensor_d.lines[3], sensor_d.x_range),
                                    )
                                {
                                    if let Some(intersection) =
                                        Line::new(sensor_a.lines[0], sensor_a.x_range).intersection(
                                            &Line::new(sensor_d.lines[3], sensor_d.x_range),
                                        )
                                    {
                                        if intersection.within(&lower, &upper) {
                                            return intersection;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // for sensor_c in sensors {
                    //     if sensor_c.within_bounds(&lower, &upper)
                    //         && sensor_b.position != sensor_c.position
                    //         && sensor_a.position != sensor_c.position
                    //     {
                    //         for sensor_d in sensors {
                    //             println!("Inner Sensor D {:?}", sensor_d.position);
                    //             if sensor_d.within_bounds(&lower, &upper)
                    //                 && sensor_c.position != sensor_d.position
                    //                 && sensor_b.position != sensor_d.position
                    //                 && sensor_a.position != sensor_d.position
                    //             {
                    //                 let a: HashSet<Position> =
                    //                     HashSet::from_iter(sensor_a.line_points());
                    //                 let b: HashSet<Position> =
                    //                     HashSet::from_iter(sensor_b.line_points());
                    //                 let c: HashSet<Position> =
                    //                     HashSet::from_iter(sensor_c.line_points());
                    //                 let d: HashSet<Position> =
                    //                     HashSet::from_iter(sensor_d.line_points());

                    //                 let ab: HashSet<Position> =
                    //                     HashSet::from_iter(a.intersection(&b).copied());
                    //                 let abc = HashSet::from_iter(ab.intersection(&c).copied());
                    //                 let unique: Vec<Position> =
                    //                     abc.intersection(&d).copied().collect();

                    //                 if unique.len() == 1
                    //                     && !sensors.iter().any(|s| s.contains(&unique[0]))
                    //                 {
                    //                     // print_sensor_outline(
                    //                     //     &lower,
                    //                     //     &upper,
                    //                     //     &vec![&sensor_a, &sensor_b, &sensor_c, &sensor_d],
                    //                     // );
                    //                     println!("{:?}", unique);
                    //                     println!(
                    //                         "{:?}",
                    //                         vec![sensor_a, sensor_b, sensor_c, sensor_d]
                    //                     );

                    //                     return unique[0];
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
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
