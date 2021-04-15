use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

struct Cart {
    row: isize,
    col: isize,
    dir: char,
    intersections: usize,
    crashed: bool,
}

impl Cart {
    fn new(row: isize, col: isize, dir: char) -> Self {
        Cart {
            row,
            col,
            dir,
            intersections: 0,
            crashed: false,
        }
    }

    fn do_move(&mut self) {
        let m = match self.dir {
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            '^' => (-1, 0),
            _ => unreachable!(),
        };
        self.row += m.0;
        self.col += m.1;
    }

    fn enter_intersection(&mut self) {
        match self.intersections % 3 {
            0 => self.turn_left(),
            2 => self.turn_right(),
            _ => (),
        }
        self.intersections += 1;
    }

    fn turn_left(&mut self) {
        self.dir = match self.dir {
            '>' => '^',
            'v' => '>',
            '<' => 'v',
            '^' => '<',
            _ => unreachable!(),
        }
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            '^' => '>',
            _ => unreachable!(),
        }
    }

    fn mark_as_crashed(&mut self) {
        self.crashed = true;
    }

    fn row(&self) -> isize {
        self.row
    }

    fn col(&self) -> isize {
        self.col
    }

    fn dir(&self) -> char {
        self.dir
    }

    fn crashed(&self) -> bool {
        self.crashed
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<Vec<char>> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn get_carts(grid: &[Vec<char>]) -> Vec<Cart> {
    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, col)| "<>^v".contains(*col))
                .map(move |(c, col)| (r, c, *col))
        })
        .map(|(r, c, dir)| Cart::new(r as isize, c as isize, dir))
        .collect()
}

fn check_collisions(carts: &[Cart]) -> Option<(isize, isize)> {
    let mut coords = HashSet::new();
    for cart in carts {
        if !coords.insert((cart.row(), cart.col())) {
            return Some((cart.row(), cart.col()));
        }
    }
    None
}

fn mark_collisions(carts: &mut [Cart]) {
    for i in 0..carts.len() {
        let (a, b) = carts.split_at_mut(i);
        let item_b = &mut b[0];
        for item_a in a {
            if item_a.row() == item_b.row() && item_a.col() == item_b.col() {
                item_a.mark_as_crashed();
                item_b.mark_as_crashed();
            }
        }
    }
}

fn solve_part1(grid: &[Vec<char>]) -> (isize, isize) {
    let mut carts = get_carts(&grid);

    loop {
        carts.sort_by_key(|c| (c.row(), c.col()));
        for i in 0..carts.len() {
            let cart = &mut carts[i];
            cart.do_move();
            match grid[cart.row() as usize][cart.col() as usize] {
                '+' => cart.enter_intersection(),
                '/' => match cart.dir() {
                    '^' | 'v' => cart.turn_right(),
                    '>' | '<' => cart.turn_left(),
                    _ => unreachable!(),
                },
                '\\' => match cart.dir() {
                    '^' | 'v' => cart.turn_left(),
                    '>' | '<' => cart.turn_right(),
                    _ => unreachable!(),
                },
                '<' | '>' | '^' | 'v' | '-' | '|' => (),
                _ => unreachable!(),
            }

            if let Some(coord) = check_collisions(&carts) {
                return coord;
            }
        }
    }
}

fn solve_part2(grid: &[Vec<char>]) -> Option<(isize, isize)> {
    let mut carts = get_carts(&grid);

    while carts.len() > 1 {
        carts.sort_by_key(|c| (c.row(), c.col()));
        for i in 0..carts.len() {
            let cart = &mut carts[i];
            cart.do_move();
            match grid[cart.row() as usize][cart.col() as usize] {
                '+' => cart.enter_intersection(),
                '/' => match cart.dir() {
                    '^' | 'v' => cart.turn_right(),
                    '>' | '<' => cart.turn_left(),
                    _ => unreachable!(),
                },
                '\\' => match cart.dir() {
                    '^' | 'v' => cart.turn_left(),
                    '>' | '<' => cart.turn_right(),
                    _ => unreachable!(),
                },
                '<' | '>' | '^' | 'v' | '-' | '|' => (),
                _ => unreachable!(),
            }
            mark_collisions(&mut carts);
        }
        carts.retain(|c| !c.crashed());
    }

    if carts.len() == 1 {
        Some((carts[0].row(), carts[0].col()))
    } else {
        None
    }
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let grid = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&grid);
    let part2 = solve_part2(&grid).unwrap();
    println!("Part 1: {},{}", part1.1, part1.0);
    println!("Part 2: {},{}", part2.1, part2.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_example1() {
        let grid = parse_input("example-part1.txt");
        assert_eq!(solve_part1(&grid), (3, 7));
    }

    #[test]
    fn test_puzzle_example2() {
        let grid = parse_input("example-part2.txt");
        assert_eq!(solve_part2(&grid), Some((4, 6)));
    }

    #[test]
    fn test_puzzle_input() {
        let grid = parse_input("input.txt");
        assert_eq!(solve_part1(&grid), (73, 65));
        assert_eq!(solve_part2(&grid), Some((66, 54)));
    }
}
