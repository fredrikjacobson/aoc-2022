use std::{collections::HashSet, hash::Hash, str::FromStr};

use crate::helper::{read_lines, ParseError};

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[test]
pub fn test_day12_pt_1() {
    let lines: Vec<String> = read_lines(12, false);
    let grid: Grid = lines.iter().map(|l| l.chars().collect()).collect();
    let start = find_start(&grid);
    let destination = find_elves(&grid);

    let steps = walk(
        HashSet::new(),
        &grid,
        &Position {
            x: start.0 as i32,
            y: start.1 as i32,
        },
        'a',
        0,
    );

    println!("Part1 score is {:?}", steps);
    assert!(false);
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
use Direction::*;

#[derive(Clone, Copy)]
enum Elevation {
    Next,
    Current,
}

use Elevation::*;

fn walk(
    visited: HashSet<Position>,
    grid: &Grid,
    position: &Position,
    elevation: char,
    steps: usize,
) -> Option<usize> {
    let next_elevation: char = if elevation == 'z' {
        'E'
    } else {
        char::from_u32(elevation as u32 + 1).unwrap()
    };

    if elevation == 'E' {
        return Some(steps);
    }

    let mut searches = Vec::new();
    for direction in vec![Up, Right, Down, Left] {
        for search_elevation in vec![Next, Current] {
            searches.push((direction, search_elevation))
        }
    }

    if steps > 700 {
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
        return None;
    }
    let mut branches: Vec<usize> = Vec::new();

    for (direction, search_elevation) in searches {
        let needle = match search_elevation {
            Next => next_elevation,
            Current => elevation,
        };
        let (x_delta, y_delta) = match direction {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        };

        if let Some(new_elevation) =
            is_elevation_match(&visited, grid, &position, x_delta, y_delta, needle)
        {
            println!(
                "Step: {} Elevation {} at position {:?} - walking to {:?} elevation {}",
                steps, elevation, position, direction, new_elevation
            );
            let mut new_visited = visited.clone();
            new_visited.insert(position.clone());
            if let Some(in_steps) = walk(
                new_visited,
                grid,
                &Position {
                    x: position.x + x_delta,
                    y: position.y + y_delta,
                },
                new_elevation,
                steps + 1,
            ) {
                branches.push(in_steps);
            }
        }
    }

    if branches.is_empty() {
        println!(
            "Step: {} Elevation {} at position {:?} - No match found for {}",
            steps, elevation, position, next_elevation
        );
        None
    } else {
        println!("OK Branches: {:?}", branches);
        branches.sort();
        Some(branches[0])
    }
}

fn is_elevation_match(
    visited: &HashSet<Position>,
    grid: &Grid,
    position: &Position,
    x_delta: i32,
    y_delta: i32,
    needle: char,
) -> Option<char> {
    let x = position.x + x_delta;
    let y = position.y + y_delta;
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    if visited.contains(&Position { x, y }) {
        None
    } else if x < 0 || y < 0 || x as usize >= num_cols || y as usize >= num_rows {
        None
    } else {
        let x = x as usize;
        let y = y as usize;

        if grid[y][x] == needle {
            Some(needle)
        } else {
            None
        }
    }
}

fn find_start(grid: &Grid) -> (usize, usize) {
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            if grid[row][col] == 'S' {
                return (col, row);
            }
        }
    }
    return (0, 0);
}

fn find_elves(grid: &Grid) -> (usize, usize) {
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            if grid[row][col] == 'E' {
                return (row, col);
            }
        }
    }
    return (0, 0);
}

#[test]
pub fn test_day12_pt_2() {
    let lines: Vec<String> = read_lines(12, false);
    println!("Part2 score is {:?}", 0);
    assert!(false);
}
