use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter::FromIterator;
use std::path::Path;
use std::process;

fn parse_input(file_name: impl AsRef<Path>) -> Vec<char> {
    fs::read_to_string(file_name)
        .unwrap()
        .trim()
        .chars()
        .collect()
}

fn opposite_polarity(a: char, b: char) -> bool {
    if a.is_ascii_uppercase() {
        a.to_ascii_lowercase() == b
    } else {
        a.to_ascii_uppercase() == b
    }
}

fn solve_part1(polymer: &[char]) -> usize {
    let mut stack = Vec::new();
    for unit in polymer {
        match stack.last() {
            Some(last) => {
                if opposite_polarity(*unit, *last) {
                    stack.pop();
                } else {
                    stack.push(*unit);
                }
            }
            None => stack.push(*unit),
        }
    }
    stack.len()
}

fn solve_part2(polymer: &[char]) -> usize {
    HashSet::<char>::from_iter(polymer.iter().map(|x| x.to_ascii_lowercase()))
        .iter()
        .map(|x| {
            solve_part1(
                &polymer
                    .iter()
                    .filter(|c| c.to_ascii_lowercase() != *x)
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        })
        .min()
        .unwrap()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let polymer = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&polymer);
    let part2 = solve_part2(&polymer);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input() {
        let polymer = parse_input("input.txt");
        assert_eq!(solve_part1(&polymer), 9116);
        assert_eq!(solve_part2(&polymer), 6890);
    }
}
