use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

use itertools::Itertools;

fn parse_input(file_name: impl AsRef<Path>) -> Vec<String> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn solve_part1(ids: &[String]) -> i32 {
    let (mut count2, mut count3) = (0, 0);
    for id in ids {
        let mut counts = HashMap::new();
        for c in id.chars() {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        if counts.values().any(|&x| x == 2) {
            count2 += 1;
        }

        if counts.values().any(|&x| x == 3) {
            count3 += 1;
        }
    }
    count2 * count3
}

fn solve_part2(ids: &[String]) -> Option<String> {
    for comb in ids.iter().combinations(2) {
        let it = comb[0].chars().zip(comb[1].chars());
        if it.clone().filter(|(a, b)| a != b).count() == 1 {
            return Some(it.filter(|(a, b)| a == b).map(|(a, _)| a).collect());
        }
    }
    None
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let ids = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&ids);
    let part2 = solve_part2(&ids);
    println!("Part 1: {}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input() {
        let ids = parse_input("input.txt");
        assert_eq!(solve_part1(&ids), 6474);
        assert_eq!(
            solve_part2(&ids),
            Some(String::from("mxhwoglxgeauywfkztndcvjqr"))
        );
    }
}
