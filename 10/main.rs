use regex::Regex;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;
use std::process;
use std::str::FromStr;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
}

#[derive(Clone)]
struct Point {
    px: isize,
    py: isize,
    vx: isize,
    vy: isize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = RE.captures(s).unwrap();
        Ok(Point {
            px: cap[1].parse().unwrap(),
            py: cap[2].parse().unwrap(),
            vx: cap[3].parse().unwrap(),
            vy: cap[4].parse().unwrap(),
        })
    }
}

impl Point {
    fn do_move(&mut self) {
        self.px += self.vx;
        self.py += self.vy;
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<Point> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| Point::from_str(l).unwrap())
        .collect()
}

fn message_received(points: &[Point]) -> bool {
    let min_x = points.iter().map(|p| p.px).min().unwrap();
    let max_x = points.iter().map(|p| p.px).max().unwrap();
    let min_y = points.iter().map(|p| p.py).min().unwrap();
    let max_y = points.iter().map(|p| p.py).max().unwrap();
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    if height > 10 {
        return false;
    }

    let mut grid = vec![vec![' '; width]; height];
    for p in points {
        grid[(p.py - min_y) as usize][(p.px - min_x) as usize] = '#';
    }

    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();

    true
}

fn solve(points: &mut [Point]) -> usize {
    let mut secs = 0;
    while !message_received(&points) {
        for p in points.iter_mut() {
            p.do_move();
        }
        secs += 1;
    }
    secs
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let mut points = parse_input(env::args().nth(1).unwrap());
    let secs = solve(&mut points);
    println!("Seconds: {}", secs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input() {
        let mut points = parse_input("input.txt");
        assert_eq!(solve(&mut points), 10867);
    }
}
