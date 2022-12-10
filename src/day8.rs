use std::{collections::HashSet, hash::Hash, str::FromStr};

use crate::helper::{read_lines, ParseError};

type Grid = Vec<Vec<u32>>;

#[test]
pub fn test_day8_pt_1() {
    let lines: Vec<String> = read_lines(8, false);

    let grid: Grid = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let rows = grid.len();
    let width = grid[0].len();
    let mut visible: Vec<u32> = Vec::new();
    for row in 1..rows - 1 {
        for col in 1..width - 1 {
            if is_visible(row, col, &grid) {
                visible.push(grid[row][col]);
            }
        }
    }

    let outside_grid = grid.len() * 2 + (grid[0].len() - 2) * 2;
    println!("Part1 score is {:?}", visible.len() + outside_grid);
    assert!(false);
}

fn is_visible(row: usize, col: usize, grid: &Grid) -> bool {
    let tree = grid[row][col];
    let mut step = 1;
    let height = grid.len();
    let width = grid[0].len();

    let mut left_visible = true;
    let mut right_visible = true;
    let mut top_visible = true;
    let mut bottom_visible = true;

    while (left_visible || right_visible || top_visible || bottom_visible)
        && (is_inside_grid(row, col, width, height, step))
    {
        if left_visible {
            if let Some(left_tree) = left_scan(row, col, step, grid) {
                left_visible = left_tree < tree;
            }
        }
        if right_visible {
            if let Some(right_tree) = right_scan(row, col, step, grid) {
                right_visible = right_tree < tree;
            }
        }
        if top_visible {
            if let Some(top_tree) = top_scan(row, col, step, grid) {
                top_visible = top_tree < tree;
            }
        }
        if bottom_visible {
            if let Some(bottom_tree) = bottom_scan(row, col, step, grid) {
                bottom_visible = bottom_tree < tree;
            }
        }

        step = step + 1;
    }

    return left_visible || right_visible || top_visible || bottom_visible;
}

fn left_scan(row: usize, col: usize, step: usize, grid: &Grid) -> Option<u32> {
    if (col as i8 - step as i8) >= 0 {
        Some(grid[row][col - step])
    } else {
        None
    }
}
fn right_scan(row: usize, col: usize, step: usize, grid: &Grid) -> Option<u32> {
    let width = grid[0].len();

    if col + step <= width - 1 {
        Some(grid[row][col + step])
    } else {
        None
    }
}
fn top_scan(row: usize, col: usize, step: usize, grid: &Grid) -> Option<u32> {
    if (row as i8 - step as i8) >= 0 {
        Some(grid[row - step][col])
    } else {
        None
    }
}

fn bottom_scan(row: usize, col: usize, step: usize, grid: &Grid) -> Option<u32> {
    let width = grid[0].len();

    if row + step <= grid.len() - 1 {
        Some(grid[row + step][col])
    } else {
        None
    }
}

fn is_inside_grid(row: usize, col: usize, width: usize, height: usize, step: usize) -> bool {
    if (col as i8 - step as i8) >= 0 {
        return true;
    } else if (col + step) <= width - 1 {
        return true;
    }
    if (row as i8 - step as i8) >= 0 {
        return true;
    } else if (row + step) <= height - 1 {
        return true;
    } else {
        return false;
    }
}

#[test]
pub fn test_day8_pt_2() {
    let lines: Vec<String> = read_lines(8, false);

    let grid: Grid = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let rows = grid.len();
    let width = grid[0].len();
    let mut max_score: u32 = 0;
    for row in 0..rows {
        for col in 0..width {
            let score = scenic_score(row, col, &grid);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Part2 score is {:?}", max_score);
    assert!(false);
}

fn scenic_score(row: usize, col: usize, grid: &Grid) -> u32 {
    let tree = grid[row][col];
    let mut step = 1;
    let height = grid.len();
    let width = grid[0].len();

    let mut left_distance = 0;
    let mut right_distance = 0;
    let mut top_distance = 0;
    let mut bottom_distance = 0;
    let mut left_visible = true;
    let mut right_visible = true;
    let mut top_visible = true;
    let mut bottom_visible = true;

    while (is_inside_grid(row, col, width, height, step)) {
        if left_visible {
            if let Some(left_tree) = left_scan(row, col, step, grid) {
                left_visible = left_tree < tree;
                left_distance = left_distance + 1;
            }
        }
        if right_visible {
            if let Some(right_tree) = right_scan(row, col, step, grid) {
                right_visible = right_tree < tree;
                right_distance = right_distance + 1;
            }
        }
        if top_visible {
            if let Some(top_tree) = top_scan(row, col, step, grid) {
                top_visible = top_tree < tree;
                top_distance = top_distance + 1;
            }
        }
        if bottom_visible {
            if let Some(bottom_tree) = bottom_scan(row, col, step, grid) {
                bottom_visible = bottom_tree < tree;
                bottom_distance = bottom_distance + 1;
            }
        }

        step = step + 1;
    }

    return left_distance * right_distance * top_distance * bottom_distance;
}
