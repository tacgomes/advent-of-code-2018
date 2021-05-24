use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
#[macro_use]
extern crate lazy_static;
use itertools::Itertools;

lazy_static! {
    static ref RE1: Regex = Regex::new(r"x=(\d+), y=(\d+)..(\d+)").unwrap();
    static ref RE2: Regex = Regex::new(r"y=(\d+), x=(\d+)..(\d+)").unwrap();
}

type Area = (usize, usize, usize, usize);

const SPRING_X: usize = 500;
const SPRING_Y: usize = 0;

#[derive(Eq, PartialEq, Clone)]
enum Cell {
    Spring,
    Sand,
    Clay,
    Flowing,
    Still,
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<Area> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| {
            if let Some(cap) = RE1.captures(l) {
                let x = cap[1].parse::<usize>().unwrap();
                let y1 = cap[2].parse::<usize>().unwrap();
                let y2 = cap[3].parse::<usize>().unwrap();
                (x, x, y1, y2)
            } else if let Some(cap) = RE2.captures(l) {
                let y = cap[1].parse::<usize>().unwrap();
                let x1 = cap[2].parse::<usize>().unwrap();
                let x2 = cap[3].parse::<usize>().unwrap();
                (x1, x2, y, y)
            } else {
                panic!("Malformatted input line: {}", l);
            }
        })
        .collect()
}

fn drip(row: usize, col: usize, xdir: isize, grid: &mut [Vec<Cell>]) -> usize {
    if grid[row][col] == Cell::Sand {
        grid[row][col] = Cell::Flowing;
    }

    if row == grid.len() - 1 || grid[row][col] == Cell::Clay {
        return col;
    }

    if grid[row + 1][col] == Cell::Sand {
        drip(row + 1, col, 0, grid);
    }

    if grid[row + 1][col] == Cell::Clay || grid[row + 1][col] == Cell::Still {
        if xdir != 0 {
            return drip(row, (col as isize + xdir) as usize, xdir, grid);
        } else {
            let left_col = drip(row, col - 1, -1, grid);
            let right_col = drip(row, col + 1, 1, grid);
            if grid[row][left_col] == Cell::Clay && grid[row][right_col] == Cell::Clay {
                for c in left_col + 1..right_col {
                    grid[row][c] = Cell::Still;
                }
            }
        }
    }

    col
}

fn display(grid: &[Vec<Cell>]) {
    for row in grid.iter() {
        for col in row.iter() {
            match col {
                Cell::Spring => print!("+"),
                Cell::Sand => print!("."),
                Cell::Clay => print!("#"),
                Cell::Flowing => print!("|"),
                Cell::Still => print!("~"),
            };
        }
        println!();
    }
    println!();
}

fn count_cells(cell: Cell, min_y: usize, grid: &[Vec<Cell>]) -> usize {
    grid.iter()
        .skip(min_y)
        .flat_map(|x| x.iter())
        .filter(|&col| *col == cell)
        .count()
}

fn solve(clay_areas: &[Area]) -> (usize, usize) {
    let min_x = clay_areas.iter().map(|t| t.0).min().unwrap();
    let max_x = clay_areas.iter().map(|t| t.1).max().unwrap();
    let min_y = clay_areas.iter().map(|t| t.2).min().unwrap();
    let max_y = clay_areas.iter().map(|t| t.3).max().unwrap();

    let mut grid = vec![vec![Cell::Sand; max_x - min_x + 2]; max_y + 1];
    grid[SPRING_Y][SPRING_X - min_x + 1] = Cell::Spring;

    for area in clay_areas {
        let (x1, x2, y1, y2) = *area;
        for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
            grid[y][x - min_x + 1] = Cell::Clay;
        }
    }

    drip(SPRING_Y + 1, SPRING_X - min_x + 1, 0, &mut grid);

    display(&grid);

    let flowing = count_cells(Cell::Flowing, min_y, &grid);
    let still = count_cells(Cell::Still, min_y, &grid);

    (flowing + still, still)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let clay_areas = parse_input(env::args().nth(1).unwrap());
    let (part1, part2) = solve(&clay_areas);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_example() {
        let clay_areas = parse_input("example.txt");
        assert_eq!(solve(&clay_areas), (57, 29));
    }

    #[test]
    fn test_puzzle_input() {
        let clay_areas = parse_input("input.txt");
        assert_eq!(solve(&clay_areas), (31471, 24169));
    }
}
