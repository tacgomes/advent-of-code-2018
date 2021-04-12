use std::env;
use std::process;

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

fn power_level(x: usize, y: usize, serial_number: usize) -> isize {
    let rack_id = x + 10;
    let hundreds_digit = |x| (x / 100) % 10;
    hundreds_digit((rack_id * y + serial_number) * rack_id) as isize - 5
}

fn solve_part1(serial_number: usize) -> (usize, usize) {
    (0..HEIGHT - 1)
        .flat_map(|sr| (0..WIDTH - 1).map(move |sc| (sr, sc)))
        .max_by_key(|&(sr, sc)| {
            (0..3)
                .flat_map(|r| (0..3).map(move |c| (r, c)))
                .map(|(r, c)| power_level(sc + c, sr + r, serial_number))
                .sum::<isize>()
        })
        .map(|(r, c)| (c, r))
        .unwrap()
}

fn solve_part2(serial_number: usize) -> (usize, usize, usize) {
    let mut sat = [[0; WIDTH + 1]; HEIGHT + 1];

    // https://en.wikipedia.org/wiki/Summed-area_table
    for r in 1..=HEIGHT {
        for c in 1..=WIDTH {
            sat[r][c] = power_level(c, r, serial_number);
            sat[r][c] += sat[r - 1][c];
            sat[r][c] += sat[r][c - 1];
            sat[r][c] -= sat[r - 1][c - 1];
        }
    }

    (0..=WIDTH)
        .flat_map(|size| {
            (1..=HEIGHT - size).flat_map(move |r| (1..=WIDTH - size).map(move |c| (r, c, size)))
        })
        .max_by_key(|&(r, c, s)| sat[r + s][c + s] - sat[r][c + s] - sat[r + s][c] + sat[r][c])
        .map(|(r, c, s)| (c + 1, r + 1, s))
        .unwrap()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} SERIAL_NUMBER", env::args().next().unwrap());
        process::exit(1);
    }

    let serial_number = env::args().nth(1).unwrap().parse().unwrap();
    let part1 = solve_part1(serial_number);
    let part2 = solve_part2(serial_number);
    println!("Part 1: {},{}", part1.0, part1.1);
    println!("Part 2: {},{},{}", part2.0, part2.1, part2.2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input() {
        assert_eq!(solve_part1(8141), (235, 16));
        assert_eq!(solve_part2(8141), (236, 227, 14));
    }
}
