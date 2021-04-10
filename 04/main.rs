use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

use chrono::prelude::*;
use counter::Counter;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\[(.+)\] (.*)").unwrap();
    static ref RE_DIGITS: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug)]
enum Event {
    StartsShift(usize),
    FallsAsleep,
    WakesUp,
}

fn parse_record(s: &str) -> (NaiveDateTime, Event) {
    let cap = RE.captures(s).unwrap();

    let datetime = NaiveDateTime::parse_from_str(&cap[1], "%Y-%m-%d %H:%M").unwrap();

    let event = if cap[2].starts_with("Guard") {
        let digits = RE_DIGITS.captures(&cap[2]).unwrap()[0].parse().unwrap();
        Event::StartsShift(digits)
    } else if &cap[2] == "falls asleep" {
        Event::FallsAsleep
    } else if &cap[2] == "wakes up" {
        Event::WakesUp
    } else {
        panic!("Invalid input: {}", &cap[2]);
    };

    (datetime, event)
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<(NaiveDateTime, Event)> {
    let mut records = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| parse_record(l))
        .collect::<Vec<_>>();
    records.sort_by(|a, b| a.0.cmp(&b.0));
    records
}

fn solve_part1(records: &[(NaiveDateTime, Event)]) -> usize {
    let (mut guard, mut asleep_minute) = (0, 0);
    let mut sleeping_times = HashMap::new();

    for (datetime, event) in records {
        match event {
            Event::StartsShift(id) => guard = *id,
            Event::FallsAsleep => asleep_minute = datetime.minute(),
            Event::WakesUp => {
                for m in asleep_minute..datetime.minute() {
                    let entry = sleeping_times.entry(guard).or_insert([0; 60]);
                    entry[m as usize] += 1;
                }
            }
        }
    }

    let best_guard = sleeping_times
        .iter()
        .max_by_key(|(_, minutes)| minutes.iter().sum::<usize>())
        .map(|(guard, _)| *guard)
        .unwrap();

    let best_minute = sleeping_times
        .get(&best_guard)
        .unwrap()
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .map(|(minute, _)| minute)
        .unwrap();

    best_guard * best_minute
}

fn solve_part2(records: &[(NaiveDateTime, Event)]) -> usize {
    let (mut guard, mut asleep_minute) = (0, 0);
    let mut minutes_to_guards = vec![vec![]; 60];

    for (datetime, event) in records {
        match event {
            Event::StartsShift(id) => guard = *id,
            Event::FallsAsleep => asleep_minute = datetime.minute(),
            Event::WakesUp => {
                for m in asleep_minute..datetime.minute() {
                    minutes_to_guards[m as usize].push(guard);
                }
            }
        }
    }

    let (best_guard, best_minute) = minutes_to_guards
        .iter()
        .enumerate()
        .filter(|(_, times)| !times.is_empty())
        .map(|(minute, guards)| {
            (
                minute,
                guards.iter().collect::<Counter<_>>().most_common_ordered()[0],
            )
        })
        .max_by_key(|(_, counter)| counter.1)
        .map(|(minute, counter)| (counter.0, minute))
        .unwrap();

    best_guard * best_minute
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let records = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&records);
    let part2 = solve_part2(&records);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_example() {
        let records = parse_input("example.txt");
        assert_eq!(solve_part1(&records), 240);
        assert_eq!(solve_part2(&records), 4455);
    }

    #[test]
    fn test_puzzle_input() {
        let records = parse_input("input.txt");
        assert_eq!(solve_part1(&records), 30630);
        assert_eq!(solve_part2(&records), 136571);
    }
}
