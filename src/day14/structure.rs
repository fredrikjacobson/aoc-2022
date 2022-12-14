use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{self, digit1, newline, one_of},
    combinator::map_res,
    error::{ErrorKind, ParseError},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult, Parser,
};

type Grid = Vec<Vec<Cell>>;

#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    Rock,
    Sand,
    Rest,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct Path {
    pub lines: Vec<Line>,
}

fn parse_path(input: &str) -> Vec<Vec<(u32, u32)>> {
    let (input, matches) = separated_list1::<_, _, _, (_, ErrorKind), _, _>(
        newline,
        separated_list1(
            tag(" -> "),
            tuple((complete::u32, preceded(tag(","), complete::u32))),
        ),
    )(input)
    .unwrap();
    matches
}

pub fn parse_paths(input: &str) -> Vec<Path> {
    parse_path(input)
        .iter()
        .map(|path| {
            let lines = path
                .windows(2)
                .map(|window| Line {
                    start: (window[0].0 as usize, window[0].1 as usize),
                    end: (window[1].0 as usize, window[1].1 as usize),
                })
                .collect();
            // println!("{:?}", lines);
            Path { lines }
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Cave {
    sand_entry: Position,
    grid: Grid,
    x_min: usize,
    x_max: usize,
}

impl ToString for Cave {
    fn to_string(&self) -> String {
        let mut lines = Vec::new();
        for (row_index, row) in self.grid.iter().enumerate() {
            let mut line = format!("Row: {}", row_index).to_owned();
            for col in self.x_min..self.x_max + 1 {
                if row_index == 0 && col == self.sand_entry.x {
                    line.push('+');
                } else {
                    let c = match row[col] {
                        Cell::Empty => '.',
                        Cell::Rest => 'o',
                        Cell::Sand => 'o',
                        Cell::Rock => '#',
                    };

                    line.push(c);
                }
            }
            lines.push(line);
        }

        lines.join("\n")
    }
}

impl Cave {
    pub fn new(paths: Vec<Path>) -> Cave {
        let min_x = find_min_x(&paths);
        let max_x = find_max_x(&paths);
        let max_y = find_max_y(&paths);

        let mut grid = vec![vec![Cell::Empty; max_x + 1]; max_y + 1];

        add_rocks(paths, &mut grid);

        Cave {
            sand_entry: Position { x: 500, y: 0 },
            grid,
            x_min: min_x,
            x_max: max_x,
        }
    }

    pub fn next(&mut self) -> bool {
        let mut sand = self.sand_entry.clone();
        let mut found = false;

        while let Some(new_position) = find_next_sand_position(&sand, &self.grid) {
            sand = new_position;
            found = true;

            if sand.x < self.x_min || self.x_max < sand.x || sand.y == self.grid.len() - 1 {
                return false;
            }
        }

        self.grid[sand.y][sand.x] = Cell::Rest;
        found
    }
}

fn find_next_sand_position(sand: &Position, grid: &Grid) -> Option<Position> {
    if let Cell::Empty = grid[sand.y + 1][sand.x] {
        Some(Position {
            x: sand.x,
            y: sand.y + 1,
        })
    } else if let Some(&Cell::Empty) = grid.get(sand.y + 1).and_then(|row| row.get(sand.x - 1)) {
        Some(Position {
            x: sand.x - 1,
            y: sand.y + 1,
        })
    } else if let Some(&Cell::Empty) = grid.get(sand.y + 1).and_then(|row| row.get(sand.x + 1)) {
        Some(Position {
            x: sand.x + 1,
            y: sand.y + 1,
        })
    } else {
        None
    }
}

fn add_rocks(paths: Vec<Path>, grid: &mut Vec<Vec<Cell>>) {
    for path in paths {
        for line in path.lines {
            // println!("{:?}", line);
            if (line.start.0 == line.end.0) {
                let mut rows = [line.start.1, line.end.1];
                rows.sort();
                for row in rows[0]..rows[1] {
                    grid[row][line.start.0] = Cell::Rock;
                }
            } else {
                let mut cols = [line.start.0, line.end.0];
                cols.sort();
                for col in cols[0]..cols[1] + 1 {
                    grid[line.start.1][col] = Cell::Rock;
                }
            }
        }
    }
}

pub fn find_max_y(paths: &Vec<Path>) -> usize {
    paths
        .iter()
        .map(|p| {
            p.lines
                .iter()
                .map(|l| l.end.1.max(l.start.1))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn find_max_x(paths: &Vec<Path>) -> usize {
    paths
        .iter()
        .map(|p| {
            p.lines
                .iter()
                .map(|l| l.end.0.max(l.start.0))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn find_min_x(paths: &Vec<Path>) -> usize {
    paths
        .iter()
        .map(|p| {
            p.lines
                .iter()
                .map(|l| l.end.0.min(l.start.0))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[test]
pub fn test_paths() {
    let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    let paths = parse_paths(input);
    println!("{:?}", paths);

    assert!(paths.len() == 2);
}
