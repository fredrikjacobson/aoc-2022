use std::{collections::HashSet, iter::FromFn, ops::Range};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{self, digit1, newline, one_of},
    combinator::{map, map_res},
    error::{ErrorKind, ParseError},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult, Parser,
};

#[test]
pub fn test_parse_sensor() {
    let input = r#"Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
    let (_, sensor) = parse_sensor(input).unwrap();
    assert!(sensor.position.x == 20);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn within(&self, lower: &Position, upper: &Position) -> bool {
        if (self.x >= lower.x && self.x <= upper.x && self.y >= lower.y && self.y <= upper.y) {
            return true;
        }
        false
    }
}
#[derive(Debug, Clone)]
pub struct Sensor {
    pub position: Position,
    pub closest_beacon: Position,
}

impl Sensor {
    pub fn search_grid(
        &self,
        lower: Position,
        upper: Position,
    ) -> impl Iterator<Item = (i32, Range<i32>)> + '_ {
        let distance = self.manhattan_distance();
        let row = self.position.y - distance;
        let mut count = -1 + (lower.y - row).max(0);
        let top_iterator = std::iter::from_fn(move || {
            count += 1;

            if count < distance && row + count <= upper.y {
                Some((
                    row + count,
                    (self.position.x - count).max(lower.x)
                        ..((self.position.x + count) + 1).min(upper.x),
                ))
            } else {
                None
            }
        });

        let mut bottom_count = -1 + (lower.y - self.position.y).max(0);
        let bottom_iterator = std::iter::from_fn(move || {
            bottom_count += 1;

            if bottom_count <= distance && self.position.y + bottom_count <= upper.y {
                Some((
                    self.position.y + bottom_count,
                    (self.position.x - (distance - bottom_count)).max(lower.x)
                        ..(self.position.x + (distance - bottom_count)).min(upper.x) + 1,
                ))
            } else {
                None
            }
        });

        top_iterator.chain(bottom_iterator)
    }

    pub fn manhattan_distance(&self) -> i32 {
        (self.position.x.abs_diff(self.closest_beacon.x)
            + self.position.y.abs_diff(self.closest_beacon.y)) as i32
    }

    pub fn within_bounds(&self, lower: &Position, upper: &Position) -> bool {
        let distance = self.manhattan_distance();
        let x_range = (self.position.x - distance, self.position.x + distance);
        let y_range = (self.position.y - distance, self.position.y + distance);

        if (x_range.0 <= lower.x && x_range.1 >= lower.x)
            || (x_range.0 >= lower.x && x_range.0 <= upper.x)
        {
            if (y_range.0 <= lower.y && y_range.1 >= lower.y)
                || (y_range.0 >= lower.y && y_range.0 <= upper.y)
            {
                return true;
            }
        }
        return false;
    }

    fn lines(&self) -> [(i32, i32); 4] {
        let mut slopes = [(0, 0); 4];
        let distance = self.manhattan_distance();
        let k = -1;
        let m = self.position.y - (distance + 1) - k * self.position.x;
        slopes[0] = (k, m);
        slopes[1] = (k, m + (distance + 1) * 2);
        let k = 1;
        let m = (self.position.y + (distance + 1)) - k * self.position.x;
        slopes[2] = (k, m);
        slopes[3] = (k, m - (distance + 1) * 2);
        slopes
    }

    pub fn intersects(&self, other: &Sensor) -> bool {
        if self.position != other.position {
            let this_slope = self.lines();
            let other_slope = other.lines();
            if !HashSet::from(this_slope)
                .intersection(&HashSet::from(other_slope))
                .collect::<Vec<&(i32, i32)>>()
                .is_empty()
            {
                return true;
            }
        }
        return false;
    }

    pub fn line_points(&self) -> impl Iterator<Item = Position> + '_ {
        let distance = self.manhattan_distance();
        let mut x = self.position.x - distance - 1;
        let mut y = self.position.y;
        let mut side = Side::None;
        std::iter::from_fn(move || {
            match side {
                Side::None => side = Side::TopLeft,
                Side::TopLeft => {
                    x += 1;
                    y -= 1;
                }
                Side::TopRight => {
                    x += 1;
                    y += 1;
                }
                Side::BottomRight => {
                    x -= 1;
                    y += 1;
                }
                Side::BottomLeft => {
                    x -= 1;
                    y -= 1;
                }
                Side::Done => return None,
            }

            match side {
                Side::TopLeft if x == self.position.x => side = Side::TopRight,
                Side::TopRight if x == self.position.x + distance + 1 => side = Side::BottomRight,
                Side::BottomRight if x == self.position.x => side = Side::BottomLeft,
                Side::BottomLeft if x == self.position.x - distance - 1 => side = Side::Done,
                _ => {}
            }

            return Some(Position { x, y });
        })
    }

    pub fn contains(&self, pos: &Position) -> bool {
        let lines = self.lines();
        if lines[0].0 * pos.x + lines[0].1 < pos.y
            && lines[1].0 * pos.x + lines[1].1 > pos.y
            && lines[2].0 * pos.x + lines[2].1 > pos.y
            && lines[3].0 * pos.x + lines[3].1 < pos.y
        {
            return true;
        }

        return false;
    }
}

enum Side {
    None,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    Done,
}

#[test]
pub fn test_slope() {
    let sensor_1 = Sensor {
        position: Position { x: 5, y: 5 },
        closest_beacon: Position { x: 5, y: 1 },
    };

    println!("{:?}", sensor_1.lines());
}

#[test]
pub fn test_search_grid() {
    let sensor = Sensor {
        position: Position { x: 8, y: 7 },
        closest_beacon: Position { x: 2, y: 10 },
    };

    let mut count = 0;
    let mut grid = vec![vec![false; 20]; 20];
    for (row, range) in sensor.search_grid(Position { x: 0, y: 0 }, Position { x: 100, y: 100 }) {
        for x in range {
            println!("{} {}", row, x);
            grid[row as usize][x as usize] = true;
            count += 1;
        }
    }

    print_grid(&grid);

    assert_eq!(count, 181);
}

#[test]
pub fn test_search_grid2() {
    let sensor = Sensor {
        position: Position { x: 20, y: 14 },
        closest_beacon: Position { x: 25, y: 17 },
    };

    let mut count = 0;
    let mut grid = vec![vec![true; 5]; 5];
    for (row, range) in sensor.search_grid(Position { x: 15, y: 15 }, Position { x: 20, y: 20 }) {
        for x in range {
            println!("{} {}", row, x);
            grid[(row - 15) as usize][(x - 15) as usize] = true;
            count += 1;
        }
    }

    print_grid(&grid);

    assert_eq!(count, 181);
}

pub fn print_grid(grid: &Vec<Vec<bool>>) {
    for (i, row) in grid.iter().enumerate() {
        let mut line = "".to_owned();
        line.push_str(&format!("Row: {: ^2}", i.to_string()));
        for col in row {
            line.push(if !*col { '#' } else { ' ' });
        }
        println!("{}", line);
    }
}

pub fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    map(
        tuple((
            preceded(tag("Sensor at x="), complete::i32),
            preceded(tag(", y="), complete::i32),
            preceded(tag(": closest beacon is at x="), complete::i32),
            preceded(tag(", y="), complete::i32),
        )),
        |(sensor_x, sensor_y, beacon_x, beacon_y)| Sensor {
            position: Position {
                x: sensor_x,
                y: sensor_y,
            },
            closest_beacon: Position {
                x: beacon_x,
                y: beacon_y,
            },
        },
    )(input)
}
