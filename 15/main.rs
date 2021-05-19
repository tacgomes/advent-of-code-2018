use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::mem;
use std::process;
use std::str::FromStr;

const HIT_POINTS: isize = 200;
const MOVES: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
    Empty,
    Wall,
    Elf(isize),
    Goblin(isize),
}

impl Cell {
    fn new(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            'G' => Cell::Goblin(HIT_POINTS),
            'E' => Cell::Elf(HIT_POINTS),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<Cell>>,
    elf_attacking_power: isize,
    goblin_attacking_power: isize,
}

impl Grid {
    fn play(&mut self, elf_attack_power: isize, goblin_attack_power: isize) -> usize {
        self.elf_attacking_power = elf_attack_power;
        self.goblin_attacking_power = goblin_attack_power;

        let mut rounds = 0;

        'outer: loop {
            println!("\nRound {}", rounds);
            self.display();

            for (r, c) in self.get_units() {
                if !self.do_turn(r as isize, c as isize) {
                    break 'outer;
                }
            }

            rounds += 1;
        }

        rounds * self.remaining_hp()
    }

    fn do_turn(&mut self, mut row: isize, mut col: isize) -> bool {
        // A unit might have died during a round leaving an empty space.
        if self.is_empty_space(row, col) {
            return true;
        }

        let target = match self.grid(row, col) {
            Cell::Elf(_) => Cell::Goblin(0),
            Cell::Goblin(_) => Cell::Elf(0),
            _ => unreachable!(),
        };

        if self.get_target_count(target) == 0 {
            return false;
        }

        if !self.target_in_range(row, col, target) {
            let new_pos = self.do_movement(row, col, target);
            row = new_pos.0;
            col = new_pos.1;
        }

        self.do_attack(row, col, target);
        true
    }

    fn do_movement(&mut self, mut row: isize, mut col: isize, target: Cell) -> (isize, isize) {
        let best_target = MOVES
            .iter()
            .map(|(mr, mc)| (row + mr, col + mc))
            .filter_map(|(r, c)| self.find_best_target(r, c, target))
            .min_by_key(|(_, _, target_row, target_col, d)| (*d, *target_row, *target_col));

        if let Some((next_row, next_col, _, _, _)) = best_target {
            self.data[next_row as usize][next_col as usize] = self.grid(row, col);
            self.data[row as usize][col as usize] = Cell::Empty;
            row = next_row;
            col = next_col;
        }

        (row, col)
    }

    fn do_attack(&mut self, row: isize, col: isize, target: Cell) -> bool {
        let min_hp = MOVES
            .iter()
            .map(|(mr, mc)| (row + mr, col + mc))
            .filter(|&(r, c)| self.is_target(r, c, target))
            .filter_map(|(r, c)| self.get_hp(r, c))
            .min_by_key(|&(_, _, hp)| hp);

        if let Some((r, c, _)) = min_hp {
            self.data[r as usize][c as usize] = match self.grid(r, c) {
                Cell::Elf(hp) => {
                    if hp - self.goblin_attacking_power > 0 {
                        Cell::Elf(hp - self.goblin_attacking_power)
                    } else {
                        Cell::Empty
                    }
                }
                Cell::Goblin(hp) => {
                    if hp - self.elf_attacking_power > 0 {
                        Cell::Goblin(hp - self.elf_attacking_power)
                    } else {
                        Cell::Empty
                    }
                }
                _ => unreachable!(),
            };
            return true;
        }

        false
    }

    fn target_in_range(&self, row: isize, col: isize, target: Cell) -> bool {
        MOVES
            .iter()
            .any(|(mr, mc)| self.is_target(row + mr, col + mc, target))
    }

    fn find_best_target(
        &self,
        row: isize,
        col: isize,
        target: Cell,
    ) -> Option<(isize, isize, isize, isize, usize)> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        if self.is_empty_space(row, col) {
            queue.push_back((row, col, 0));
        }

        while !queue.is_empty() {
            let (r, c, d) = queue.pop_front().unwrap();

            if !visited.insert((r, c)) || !self.is_empty_space(r, c) {
                continue;
            }

            if self.target_in_range(r, c, target) {
                return Some((row, col, r, c, d));
            }

            queue.extend(MOVES.iter().map(|&(mr, mc)| (r + mr, c + mc, d + 1)));
        }

        None
    }

    fn get_units(&self) -> Vec<(isize, isize)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, cell)| (r, c, cell)))
            .filter(|(_, _, cell)| Self::is_unit(cell))
            .map(|(r, c, _)| (r as isize, c as isize))
            .collect()
    }

    fn get_target_count(&self, target: Cell) -> usize {
        self.data
            .iter()
            .flat_map(|x| x.iter())
            .filter(|&x| mem::discriminant(x) == mem::discriminant(&target))
            .count()
    }

    fn remaining_hp(&self) -> usize {
        self.data
            .iter()
            .flat_map(|x| x.iter())
            .map(|x| match x {
                Cell::Goblin(hp) => *hp as usize,
                Cell::Elf(hp) => *hp as usize,
                _ => 0,
            })
            .sum()
    }

    fn get_hp(&self, row: isize, col: isize) -> Option<(isize, isize, isize)> {
        match self.grid(row, col) {
            Cell::Elf(hp) => Some((row, col, hp)),
            Cell::Goblin(hp) => Some((row, col, hp)),
            _ => None,
        }
    }

    fn grid(&self, row: isize, col: isize) -> Cell {
        self.data[row as usize][col as usize]
    }

    fn is_empty_space(&self, row: isize, col: isize) -> bool {
        self.grid(row, col) == Cell::Empty
    }

    fn is_target(&self, row: isize, col: isize, target: Cell) -> bool {
        mem::discriminant(&self.grid(row, col)) == mem::discriminant(&target)
    }

    fn is_unit(cell: &Cell) -> bool {
        matches!(cell, Cell::Elf(_)) || matches!(cell, Cell::Goblin(_))
    }

    fn display(&self) {
        for r in 0..self.data.len() {
            for c in 0..self.data[0].len() {
                match self.data[r][c] {
                    Cell::Wall => print!("#"),
                    Cell::Empty => print!("."),
                    Cell::Elf(_) => print!("E"),
                    Cell::Goblin(_) => print!("G"),
                }
            }
            print!(" ");
            for c in 0..self.data[0].len() {
                match self.data[r][c] {
                    Cell::Elf(hp) => print!(" E({})", hp),
                    Cell::Goblin(hp) => print!(" G({})", hp),
                    _ => (),
                }
            }
            println!()
        }
    }
}

fn solve_part1(grid: &Grid) -> usize {
    grid.clone().play(3, 3)
}

fn solve_part2(grid: &Grid) -> usize {
    let num_elves_before = grid.get_target_count(Cell::Elf(0));
    let mut elf_attack_power = 4;
    loop {
        let mut grid = grid.clone();
        let result = grid.play(elf_attack_power, 3);
        let num_elves_after = grid.get_target_count(Cell::Elf(0));
        if num_elves_before == num_elves_after {
            return result;
        }
        elf_attack_power += 1;
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            data: s
                .lines()
                .map(|l| l.chars().map(Cell::new).collect())
                .collect::<Vec<_>>(),
            elf_attacking_power: 0,
            goblin_attacking_power: 0,
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
        let input = fs::read_to_string("example1.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();
        assert_eq!(solve_part1(&grid), 27730);
        assert_eq!(solve_part2(&grid), 4988);
    }

    #[test]
    fn test_puzzle_example2() {
        let input = fs::read_to_string("example2.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();
        assert_eq!(solve_part1(&grid), 36334);
        assert_eq!(solve_part2(&grid), 29064);
    }

    #[test]
    fn test_puzzle_example3() {
        let input = fs::read_to_string("example3.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();
        assert_eq!(solve_part1(&grid), 39514);
        assert_eq!(solve_part2(&grid), 31284);
    }

    #[test]
    fn test_puzzle_example4() {
        let input = fs::read_to_string("example4.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();
        assert_eq!(solve_part1(&grid), 27755);
        assert_eq!(solve_part2(&grid), 3478);
    }

    #[test]
    fn test_puzzle_example5() {
        let input = fs::read_to_string("example5.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();
        assert_eq!(solve_part1(&grid), 28944);
        assert_eq!(solve_part2(&grid), 6474);
    }

    #[test]
    fn test_puzzle_example6() {
        let input = fs::read_to_string("example6.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();
        assert_eq!(solve_part1(&grid), 18740);
        assert_eq!(solve_part2(&grid), 1140);
    }

    #[test]
    fn test_puzzle_input() {
        let input = fs::read_to_string("input.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();
        assert_eq!(solve_part1(&grid), 198744);
        assert_eq!(solve_part2(&grid), 66510);
    }
}
