use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn parse_input(file_name: impl AsRef<Path>) -> Vec<usize> {
    fs::read_to_string(file_name)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn walk_tree_part1(nums: &[usize], mut metadata: &mut Vec<usize>) -> usize {
    let (num_childs, num_meta) = (nums[0], nums[1]);
    let mut index = 2;

    for _ in 0..num_childs {
        index += walk_tree_part1(&nums[index..], &mut metadata);
    }

    metadata.extend(&nums[index..index + num_meta]);

    index + num_meta
}

fn walk_tree_part2(nums: &[usize]) -> (usize, usize) {
    let (num_childs, num_meta) = (nums[0], nums[1]);
    let (mut index, mut child_sums) = (2, vec![]);

    for _ in 0..num_childs {
        let (consumed, sum) = walk_tree_part2(&nums[index..]);
        index += consumed;
        child_sums.push(sum);
    }

    let sum = if num_childs == 0 {
        nums[index..].iter().take(num_meta).sum()
    } else {
        (index..index + num_meta)
            .map(|i| nums[i])
            .filter(|&i| i > 0 && i <= child_sums.len())
            .map(|i| child_sums[i - 1])
            .sum()
    };

    (index + num_meta, sum)
}

fn solve_part1(nums: &[usize]) -> usize {
    let mut metadata = vec![];
    walk_tree_part1(nums, &mut metadata);
    metadata.iter().sum()
}

fn solve_part2(nums: &[usize]) -> usize {
    walk_tree_part2(nums).1
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let nums = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&nums);
    let part2 = solve_part2(&nums);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_example() {
        let nums = parse_input("example.txt");
        assert_eq!(solve_part1(&nums), 138);
        assert_eq!(solve_part2(&nums), 66);
    }

    #[test]
    fn test_puzzle_input() {
        let nums = parse_input("input.txt");
        assert_eq!(solve_part1(&nums), 42472);
        assert_eq!(solve_part2(&nums), 21810);
    }
}
