use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

const PART_1_ITERS: usize = 10;
const PART_2_ITERS: usize = 1000000000;

const NEIGHS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Cell {
    Open,
    Tree,
    Lumberyard,
}

impl Cell {
    fn new(c: char) -> Self {
        match c {
            '.' => Cell::Open,
            '|' => Cell::Tree,
            '#' => Cell::Lumberyard,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<Cell>>,
}

impl Grid {
    fn iterate(&mut self) {
        let mut new_data = self.data.clone();

        for (r, row) in self.data.iter().enumerate() {
            for (c, _col) in row.iter().enumerate() {
                new_data[r][c] = match self.data[r][c] {
                    Cell::Open => {
                        if self.count_neighs(r as isize, c as isize, Cell::Tree) >= 3 {
                            Cell::Tree
                        } else {
                            Cell::Open
                        }
                    }
                    Cell::Tree => {
                        if self.count_neighs(r as isize, c as isize, Cell::Lumberyard) >= 3 {
                            Cell::Lumberyard
                        } else {
                            Cell::Tree
                        }
                    }
                    Cell::Lumberyard => {
                        if self.count_neighs(r as isize, c as isize, Cell::Lumberyard) >= 1
                            && self.count_neighs(r as isize, c as isize, Cell::Tree) >= 1
                        {
                            Cell::Lumberyard
                        } else {
                            Cell::Open
                        }
                    }
                }
            }
        }

        self.data = new_data;
    }

    fn find_cycle(&mut self) -> (usize, usize) {
        let mut cache = HashMap::new();
        let mut step = 0;
        loop {
            if let Some(cycle_start) = cache.insert(self.data.clone(), step) {
                break (cycle_start, step);
            }
            self.iterate();
            step += 1;
        }
    }

    fn count_neighs(&self, row: isize, col: isize, cell: Cell) -> usize {
        NEIGHS
            .iter()
            .map(|(r, c)| (row + r, col + c))
            .filter(|&(r, c)| self.valid_pos(r, c))
            .filter(|&(r, c)| self.data[r as usize][c as usize] == cell)
            .count()
    }

    fn count_trees(&self) -> usize {
        self.data
            .iter()
            .flat_map(|x| x.iter())
            .filter(|&&x| x == Cell::Tree)
            .count()
    }

    fn count_lumberyards(&self) -> usize {
        self.data
            .iter()
            .flat_map(|x| x.iter())
            .filter(|&&x| x == Cell::Lumberyard)
            .count()
    }

    fn valid_pos(&self, row: isize, col: isize) -> bool {
        row >= 0 && col >= 0 && row < self.data.len() as isize && col < self.data[0].len() as isize
    }
}

fn solve_part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();

    for _ in 0..PART_1_ITERS {
        grid.iterate();
    }

    grid.count_trees() * grid.count_lumberyards()
}

fn solve_part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let (cycle_start, cycle_end) = grid.find_cycle();
    let cycle_len = cycle_end - cycle_start;
    let remaining = (PART_2_ITERS - cycle_start) % cycle_len;

    for _ in 0..remaining {
        grid.iterate();
    }

    grid.count_trees() * grid.count_lumberyards()
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            data: s
                .lines()
                .map(|l| l.chars().map(Cell::new).collect())
                .collect::<Vec<_>>(),
        })
    }
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let grid = Grid::from_str(&input).unwrap();
    let part1 = solve_part1(&grid);
    let part2 = solve_part2(&grid);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_example1() {
        let input = fs::read_to_string("example.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();
        assert_eq!(solve_part1(&grid), 1147);
    }

    #[test]
    fn test_puzzle_input() {
        let input = fs::read_to_string("input.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();
        assert_eq!(solve_part1(&grid), 638400);
        assert_eq!(solve_part2(&grid), 195952);
    }
}
