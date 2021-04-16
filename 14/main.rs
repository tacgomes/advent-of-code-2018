use std::env;
use std::process;

fn digits(mut num: usize) -> Vec<usize> {
    let mut digits = vec![];
    loop {
        digits.push(num % 10);
        num /= 10;
        if num == 0 {
            break;
        }
    }
    digits.reverse();
    digits
}

fn solve_part1(num_recipes: usize) -> String {
    let mut scoreboard = vec![3, 7];
    let (mut elf1, mut elf2) = (0, 1);

    while scoreboard.len() < num_recipes + 10 {
        let new_recipes = digits(scoreboard[elf1] + scoreboard[elf2]);
        scoreboard.extend(new_recipes.iter());
        elf1 = (elf1 + 1 + scoreboard[elf1]) % scoreboard.len();
        elf2 = (elf2 + 1 + scoreboard[elf2]) % scoreboard.len();
    }

    scoreboard
        .iter()
        .skip(num_recipes)
        .take(10)
        .map(|x| x.to_string())
        .collect()
}

fn solve_part2(target_score: usize) -> usize {
    let mut scoreboard = vec![3, 7];
    let (mut elf1, mut elf2) = (0, 1);
    let target_score = digits(target_score);

    loop {
        let new_recipes = digits(scoreboard[elf1] + scoreboard[elf2]);
        scoreboard.extend(new_recipes.iter());
        elf1 = (elf1 + 1 + scoreboard[elf1]) % scoreboard.len();
        elf2 = (elf2 + 1 + scoreboard[elf2]) % scoreboard.len();

        for i in 0..new_recipes.len() {
            let idx = scoreboard.len() as isize - target_score.len() as isize - i as isize;
            if idx >= 0 && scoreboard[idx as usize..scoreboard.len() - i] == target_score[..] {
                return idx as usize;
            }
        }
    }
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let input = env::args().nth(1).unwrap().parse().unwrap();
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input() {
        assert_eq!(solve_part1(702831), "1132413111");
        assert_eq!(solve_part2(702831), 20340232);
    }
}
