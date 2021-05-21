use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;
use std::process;
use std::str::FromStr;
#[macro_use]
extern crate lazy_static;

type Regs = Vec<usize>;
type OpFn = fn(&[usize], &Instr) -> Vec<usize>;

const OPERATIONS: [OpFn; 16] = [
    addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr,
];

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

struct Instr {
    opcode: usize,
    in1: usize,
    in2: usize,
    out: usize,
}

impl Instr {
    fn new(opcode: usize, in1: usize, in2: usize, out: usize) -> Self {
        Instr {
            opcode,
            in1,
            in2,
            out,
        }
    }
}

impl FromStr for Instr {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut operands = s.split(' ').filter_map(|digits| digits.parse().ok());

        Ok(Instr::new(
            operands.next().unwrap(),
            operands.next().unwrap(),
            operands.next().unwrap(),
            operands.next().unwrap(),
        ))
    }
}

struct Sample {
    regs_before: Regs,
    regs_after: Regs,
    instr: Instr,
}

impl FromStr for Sample {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();

        let regs_before = RE
            .find_iter(iter.next().unwrap())
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();

        let mut operands = RE
            .find_iter(iter.next().unwrap())
            .filter_map(|digits| digits.as_str().parse().ok());

        let regs_after = RE
            .find_iter(iter.next().unwrap())
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();

        Ok(Sample {
            regs_before,
            regs_after,
            instr: Instr::new(
                operands.next().unwrap(),
                operands.next().unwrap(),
                operands.next().unwrap(),
                operands.next().unwrap(),
            ),
        })
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> (Vec<Sample>, Vec<Instr>) {
    let input = fs::read_to_string(file_name).unwrap();

    let mut iter = input.split("\n\n\n");

    let samples = iter
        .next()
        .unwrap()
        .split("\n\n")
        .map(|l| Sample::from_str(l).unwrap())
        .collect();

    let instrs = iter
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .map(|l| Instr::from_str(l).unwrap())
        .collect();

    (samples, instrs)
}

fn addr(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = regs[instr.in1] + regs[instr.in2];
    regs
}

fn addi(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = regs[instr.in1] + instr.in2;
    regs
}

fn mulr(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = regs[instr.in1] * regs[instr.in2];
    regs
}

fn muli(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = regs[instr.in1] * instr.in2;
    regs
}

fn banr(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = regs[instr.in1] & regs[instr.in2];
    regs
}

fn bani(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = regs[instr.in1] & instr.in2;
    regs
}

fn borr(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = regs[instr.in1] | regs[instr.in2];
    regs
}

fn bori(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = regs[instr.in1] | instr.in2;
    regs
}

fn setr(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = regs[instr.in1];
    regs
}

fn seti(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = instr.in1;
    regs
}

fn gtir(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = if instr.in1 > regs[instr.in2] { 1 } else { 0 };
    regs
}

fn gtri(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = if regs[instr.in1] > instr.in2 { 1 } else { 0 };
    regs
}

fn gtrr(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = if regs[instr.in1] > regs[instr.in2] {
        1
    } else {
        0
    };
    regs
}

fn eqir(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = if instr.in1 == regs[instr.in2] { 1 } else { 0 };
    regs
}

fn eqri(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = if regs[instr.in1] == instr.in2 { 1 } else { 0 };
    regs
}

fn eqrr(state: &[usize], instr: &Instr) -> Regs {
    let mut regs = state.to_owned();
    regs[instr.out] = if regs[instr.in1] == regs[instr.in2] {
        1
    } else {
        0
    };
    regs
}

fn count_possible_operations(s: &Sample) -> usize {
    OPERATIONS
        .iter()
        .filter(|op| op(&s.regs_before, &s.instr) == s.regs_after)
        .count()
}

fn solve_part1(samples: &[Sample]) -> usize {
    samples
        .iter()
        .filter(|s| count_possible_operations(&s) >= 3)
        .count()
}

fn solve_part2(samples: &[Sample], instrs: &[Instr]) -> usize {
    let mut opcodes = vec![HashSet::new(); 16];

    for s in samples {
        for (index, op) in OPERATIONS.iter().enumerate() {
            if op(&s.regs_before, &s.instr) == s.regs_after {
                opcodes[index].insert(s.instr.opcode);
            }
        }
    }

    let mut operations = OPERATIONS;

    while let Some(index1) = opcodes.iter().position(|x| x.len() == 1) {
        let index2 = *opcodes[index1].iter().next().unwrap();

        operations[index2] = OPERATIONS[index1];

        for candidates in opcodes.iter_mut() {
            candidates.remove(&index2);
        }
    }

    let mut regs = vec![0; 4];

    for instr in instrs {
        regs = operations[instr.opcode](&regs, &instr);
    }

    regs[0]
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let (samples, instrs) = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&samples);
    let part2 = solve_part2(&samples, &instrs);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input() {
        let (samples, instrs) = parse_input("input.txt");
        assert_eq!(solve_part1(&samples), 521);
        assert_eq!(solve_part2(&samples, &instrs), 594);
    }
}
