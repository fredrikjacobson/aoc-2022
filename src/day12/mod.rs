use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    str::FromStr,
};

use crate::helper::{read_lines, ParseError};

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[test]
pub fn test_day12_pt_1() {
    day12_pt_1();
    assert!(false);
}

pub fn day12_pt_1() {
    let lines: Vec<String> = read_lines(12, false);
    let (grid, start, destination) = extract_grid(lines);
    let steps = shortest_path(&grid, start, &destination).unwrap();
    println!("Part1 score is {:?}", steps);
}

fn extract_grid(lines: Vec<String>) -> (Vec<Vec<char>>, Position, Position) {
    let mut grid: Grid = lines.iter().map(|l| l.chars().collect()).collect();
    let start = find_start(&grid);
    let destination = find_elves(&grid);
    grid[start.y as usize][start.x as usize] = 'a';
    grid[destination.y as usize][destination.x as usize] = 'z';
    (grid, start, destination)
}

fn shortest_path(grid: &Grid, start: Position, destination: &Position) -> Option<i32> {
    println!("Start: {:?}", start);
    println!("Elves: {:?}", destination);

    let mut queue = VecDeque::new();
    queue.push_back(((start.x as usize, start.y as usize), 0));
    let mut visited = HashSet::<Position>::new();
    while let Some(((x, y), len)) = queue.pop_front() {
        if (y, x) == (destination.y as usize, destination.x as usize) {
            return Some(len);
        }
        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if let Some(&square) = grid.get(ny).and_then(|row| row.get(nx)) {
                if grid[y][x] as u32 + 1 >= square as u32
                    && !visited.contains(&Position {
                        x: nx as i32,
                        y: ny as i32,
                    })
                {
                    visited.insert(Position {
                        x: nx as i32,
                        y: ny as i32,
                    });

                    queue.push_back(((nx, ny), len + 1));
                }
            }
        }
    }
    None
}

fn print_grid(grid: &Vec<Vec<char>>, visited: &HashSet<Position>, elevation: char) {
    for row in 0..grid.len() {
        let mut line = " ".repeat(grid[0].len());
        for col in 0..grid[0].len() {
            if visited.contains(&Position {
                x: col as i32,
                y: row as i32,
            }) {
                line.insert(col, '#');
            }
        }
        println!("Row: {:02} {}", row, line);
    }
    println!("Elevation: {}", elevation);
}

fn find_start(grid: &Grid) -> Position {
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            if grid[row][col] == 'S' {
                return Position {
                    x: col as i32,
                    y: row as i32,
                };
            }
        }
    }
    return Position { x: 0, y: 0 };
}

fn find_elves(grid: &Grid) -> Position {
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            if grid[row][col] == 'E' {
                return Position {
                    x: col as i32,
                    y: row as i32,
                };
            }
        }
    }
    return Position { x: 0, y: 0 };
}

#[test]
pub fn test_day12_pt_2() {
    let lines: Vec<String> = read_lines(12, false);
    let (grid, start, destination) = extract_grid(lines);

    let mut shortest = 10000;
    for start_y in 0..grid.len() {
        let steps = shortest_path(
            &grid,
            Position {
                x: 0,
                y: start_y as i32,
            },
            &destination,
        )
        .unwrap();
        if steps < shortest {
            shortest = steps;
        }
    }

    println!("Part2 score is {:?}", shortest);
    assert!(false);
}
