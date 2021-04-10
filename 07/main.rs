use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[macro_use]
extern crate lazy_static;

// const STEP_SECS: usize = 0;
// const NUM_WORKS: usize = 2;
const STEP_SECS: usize = 60;
const NUM_WORKS: usize = 5;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
}

type Dependencies = HashMap<char, Vec<char>>;

fn parse_input(file_name: impl AsRef<Path>) -> Dependencies {
    let text = fs::read_to_string(file_name).unwrap();
    let mut deps = Dependencies::new();
    for caps in RE.captures_iter(&text) {
        let (a, b) = (
            caps[1].chars().next().unwrap(),
            caps[2].chars().next().unwrap(),
        );
        deps.entry(b).or_insert_with(Vec::new).push(a);
        deps.entry(a).or_insert_with(Vec::new);
    }
    deps
}

fn step_seconds(c: char) -> usize {
    c as usize - 'A' as usize + 1 + STEP_SECS
}

fn solve_part1(deps: &Dependencies) -> String {
    let mut deps = deps.clone();
    let mut result = String::new();

    while result.len() != deps.len() {
        let mut candidates = deps
            .iter()
            .filter(|(_, v)| v.is_empty())
            .filter(|(&k, _)| !result.chars().any(|c| c == k))
            .map(|(k, _)| k)
            .cloned()
            .collect::<Vec<_>>();

        candidates.sort_unstable();
        result.push(candidates[0]);

        for v in deps.values_mut() {
            v.retain(|&x| x != candidates[0]);
        }
    }

    result
}

fn solve_part2(deps: &Dependencies) -> usize {
    let mut deps = deps.clone();
    let mut secs = 0;
    let mut steps_done = HashSet::new();
    let mut workers: [Option<(char, usize)>; NUM_WORKS] = [None; NUM_WORKS];

    while steps_done.len() != deps.len() {
        let mut candidates = deps
            .iter()
            .filter(|(_, v)| v.is_empty())
            .filter(|(k, _)| !steps_done.contains(*k))
            .filter(|(&k, _)| !workers.iter().filter_map(|&x| x).any(|x| x.0 == k))
            .map(|(k, _)| k)
            .cloned()
            .collect::<Vec<_>>();

        candidates.sort_unstable();

        let mut iter = workers.iter_mut().filter(|x| x.is_none());
        for candidate in candidates {
            if let Some(worker) = iter.next() {
                *worker = Some((candidate, step_seconds(candidate)));
            }
        }

        secs += 1;

        for worker in workers.iter_mut() {
            match worker {
                Some((step, 1)) => {
                    for v in deps.values_mut() {
                        v.retain(|x| x != step);
                    }
                    steps_done.insert(*step);
                    *worker = None;
                }
                Some((_, counter)) => {
                    *counter -= 1;
                }
                _ => (),
            }
        }
    }

    secs
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let deps = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&deps);
    let part2 = solve_part2(&deps);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input() {
        let deps = parse_input("input.txt");
        assert_eq!(
            solve_part1(&deps),
            String::from("JNOIKSYABEQRUVWXGTZFDMHLPC")
        );
        assert_eq!(solve_part2(&deps), 1099);
    }
}
