use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;
use std::process;
use std::str::FromStr;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}

#[derive(Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

type Overlaps = HashMap<(u32, u32), Vec<u32>>;

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = RE.captures(s).unwrap();
        Ok(Claim {
            id: cap[1].parse().unwrap(),
            x: cap[2].parse().unwrap(),
            y: cap[3].parse().unwrap(),
            w: cap[4].parse().unwrap(),
            h: cap[5].parse().unwrap(),
        })
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<Claim> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| Claim::from_str(l).unwrap())
        .collect()
}

fn find_overlaps(claims: &[Claim]) -> HashMap<(u32, u32), Vec<u32>> {
    let mut overlaps = HashMap::new();
    for claim in claims {
        for (x, y) in (0..claim.w).cartesian_product(0..claim.h) {
            overlaps
                .entry((claim.x + x, claim.y + y))
                .or_insert_with(Vec::new)
                .push(claim.id);
        }
    }
    overlaps
}

fn solve_part1(overlaps: &Overlaps) -> usize {
    overlaps.values().filter(|&x| x.len() > 1).count()
}

fn solve_part2(claims: &[Claim], overlaps: &Overlaps) -> Option<u32> {
    for claim in claims {
        if (0..claim.w)
            .cartesian_product(0..claim.h)
            .all(|(x, y)| overlaps.get(&(claim.x + x, claim.y + y)).unwrap().len() == 1)
        {
            return Some(claim.id);
        }
    }
    None
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let claims = parse_input(env::args().nth(1).unwrap());
    let overlaps = find_overlaps(&claims);
    let part1 = solve_part1(&overlaps);
    let part2 = solve_part2(&claims, &overlaps);
    println!("Part 1: {}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input() {
        let claims = parse_input("input.txt");
        let overlaps = find_overlaps(&claims);
        assert_eq!(solve_part1(&overlaps), 98005);
        assert_eq!(solve_part2(&claims, &overlaps), Some(331));
    }
}
