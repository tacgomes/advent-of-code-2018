use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

const NUM_GENS_PART1: usize = 20;
const NUM_GENS_PART2: usize = 50000000000;
const CONVERGENCE: usize = 10;

type Rule = (Vec<char>, char);

fn parse_input(file_name: impl AsRef<Path>) -> (Vec<char>, Vec<Rule>) {
    let text = fs::read_to_string(file_name).unwrap();
    let mut iter = text.lines();

    let initial_state = iter
        .next()
        .unwrap()
        .chars()
        .skip("initial state: ".len())
        .collect();

    let rules = iter
        .skip(1)
        .map(|l| {
            let mut parts = l.split("=>");
            (
                parts.next().unwrap().trim().chars().collect(),
                parts.next().unwrap().trim().chars().next().unwrap(),
            )
        })
        .collect();

    (initial_state, rules)
}

fn rule_matches(rule: &Rule, i: usize, pots: &VecDeque<char>) -> bool {
    rule.0[0] == pots[i - 2]
        && rule.0[1] == pots[i - 1]
        && rule.0[2] == pots[i]
        && rule.0[3] == pots[i + 1]
        && rule.0[4] == pots[i + 2]
}

fn calculate_score(pots: &VecDeque<char>, gen: usize) -> isize {
    pots.iter()
        .enumerate()
        .filter(|&(_, &x)| x == '#')
        .map(|(i, _)| i as isize - (2 + gen as isize))
        .sum()
}

fn solve(initial_state: &[char], rules: &[Rule], num_gens: usize) -> isize {
    let mut pots = VecDeque::new();
    pots.extend(&['.', '.', '.']);
    pots.extend(initial_state.iter());
    pots.extend(&['.', '.', '.']);

    let mut counts = HashMap::new();
    let mut last_score = 0;

    for gen in 1..=num_gens {
        let copy = pots.clone();
        for (i, pot) in pots.iter_mut().enumerate().skip(2).take(copy.len() - 4) {
            *pot = if let Some(r) = rules.iter().find(|r| rule_matches(r, i, &copy)) {
                r.1
            } else {
                '.'
            }
        }

        let score = calculate_score(&pots, gen);
        let diff_count = counts.entry(score - last_score).or_insert(0);

        if *diff_count > CONVERGENCE {
            last_score = (score - last_score) * (num_gens - gen) as isize + score;
            break;
        } else {
            last_score = score;
            *diff_count += 1;
        }

        pots.push_front('.');
        pots.push_back('.');
    }

    last_score
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let (initial_state, rules) = parse_input(env::args().nth(1).unwrap());
    let part1 = solve(&initial_state, &rules, NUM_GENS_PART1);
    let part2 = solve(&initial_state, &rules, NUM_GENS_PART2);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_example() {
        let (initial_state, rules) = parse_input("example.txt");
        assert_eq!(solve(&initial_state, &rules, NUM_GENS_PART1), 325);
        assert_eq!(solve(&initial_state, &rules, NUM_GENS_PART2), 999999999374);
    }

    #[test]
    fn test_puzzle_input() {
        let (initial_state, rules) = parse_input("input.txt");
        assert_eq!(solve(&initial_state, &rules, NUM_GENS_PART1), 3241);
        assert_eq!(solve(&initial_state, &rules, NUM_GENS_PART2), 2749999999911);
    }
}
