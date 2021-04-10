use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

const MAX_DISTANCE: usize = 10000;

type Point = (isize, isize);
type Coords = HashSet<(isize, isize)>;

fn parse_input(file_name: impl AsRef<Path>) -> Coords {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split(',');
            (
                parts.next().unwrap().trim().parse().unwrap(),
                parts.next().unwrap().trim().parse().unwrap(),
            )
        })
        .map(|c| (c.1, c.0))
        .collect()
}

fn bounds(coords: &Coords) -> Point {
    let max_r = coords.iter().map(|x| x.0).max().unwrap() + 1;
    let max_c = coords.iter().map(|x| x.1).max().unwrap() + 1;
    (max_r, max_c)
}

fn manhattan_distance(p1: Point, p2: Point) -> usize {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as usize
}

fn find_nearest_coord(pos: Point, coords: &Coords) -> Option<Point> {
    let distances = coords
        .iter()
        .map(|&coord| (coord, manhattan_distance(coord, pos)));

    let (nearest, distance) = distances.clone().min_by_key(|x| x.1).unwrap();

    if distances.filter(|&x| x.1 == distance).count() > 1 {
        return None;
    }

    Some(nearest)
}

fn visit(
    pos: Point,
    coord: Point,
    locations: &[Vec<Point>],
    visited: &mut Coords,
) -> Option<usize> {
    if pos.0 < 0
        || pos.0 >= locations.len() as isize
        || pos.1 < 0
        || pos.1 >= locations[0].len() as isize
    {
        return None;
    }

    if !visited.insert(pos) {
        return Some(0);
    }

    if locations[pos.0 as usize][pos.1 as usize] != coord {
        return Some(0);
    }

    let mut size = 1;
    for mov in &[(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let newpos = (pos.0 + mov.0, pos.1 + mov.1);
        match visit(newpos, coord, locations, visited) {
            Some(n) => size += n,
            None => return None,
        }
    }

    Some(size)
}

fn calculate_total_distance(pos: Point, coords: &Coords) -> usize {
    coords
        .iter()
        .map(|&coord| manhattan_distance(coord, pos))
        .sum()
}

fn solve_part1(coords: &Coords) -> usize {
    let (max_r, max_c) = bounds(&coords);
    let mut locations = vec![vec![(isize::MAX, isize::MAX); max_c as usize]; max_r as usize];

    for (r, row) in locations.iter_mut().enumerate() {
        for (c, col) in row.iter_mut().enumerate() {
            if let Some(nearest) = find_nearest_coord((r as isize, c as isize), coords) {
                *col = nearest;
            }
        }
    }

    coords
        .iter()
        .map(|&c| visit(c, c, &locations, &mut HashSet::new()))
        .filter_map(|x| x)
        .max()
        .unwrap()
}

fn solve_part2(coords: &Coords) -> usize {
    let (max_r, max_c) = bounds(&coords);
    (0..max_r)
        .flat_map(|r| (0..max_c).map(move |c| (r, c)))
        .map(|(r, c)| calculate_total_distance((r as isize, c as isize), coords))
        .filter(|&d| d < MAX_DISTANCE)
        .count()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let coords = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&coords);
    let part2 = solve_part2(&coords);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input() {
        let coords = parse_input("input.txt");
        assert_eq!(solve_part1(&coords), 4342);
        assert_eq!(solve_part2(&coords), 42966);
    }
}
