use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn parse_input(file_name: impl AsRef<Path>) -> Vec<i32> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn solve_part1(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn solve_part2(values: &[i32]) -> i32 {
    let mut set = HashSet::new();
    let mut freq = 0;
    loop {
        for freq_change in values {
            freq += freq_change;
            if !set.insert(freq) {
                return freq;
            }
        }
    }
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let values = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&values);
    let part2 = solve_part2(&values);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input() {
        let values = parse_input("input.txt");
        assert_eq!(solve_part1(&values), 547);
        assert_eq!(solve_part2(&values), 76414);
    }
}
