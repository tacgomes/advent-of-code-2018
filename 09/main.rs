use std::env;
use std::process;

const SPECIAL_MARBLE: usize = 23;

#[derive(Clone)]
struct Node {
    prev: usize,
    next: usize,
}

impl Node {
    fn new(prev: usize, next: usize) -> Self {
        Node { prev, next }
    }
}

fn solve(num_players: usize, num_marbles: usize) -> usize {
    let mut scores = vec![0; num_players];
    let mut current = 1;
    let mut player = 0;

    let mut ring = vec![Node::new(0, 0); num_marbles + 1];
    ring[0] = Node::new(1, 1);
    ring[1] = Node::new(0, 0);

    for marble in 2..=num_marbles {
        if marble % SPECIAL_MARBLE == 0 {
            let mut ptr = current;
            for _ in 0..7 {
                ptr = ring[ptr].prev;
            }
            let prev = ring[ptr].prev;
            let next = ring[ptr].next;
            ring[prev].next = next;
            ring[next].prev = prev;
            current = next;
            scores[player] += marble + ptr;
        } else {
            let prev = ring[current].next;
            let next = ring[prev].next;
            ring[prev].next = marble;
            ring[next].prev = marble;
            ring[marble] = Node::new(prev, next);
            current = marble;
        }
        player = (player + 1) % num_players;
    }

    *scores.iter().max().unwrap()
}

fn main() {
    if env::args().count() != 3 {
        eprintln!(
            "USAGE: {} NUM_PLAYERS NUM_MARBLES",
            env::args().next().unwrap()
        );
        process::exit(1);
    }
    let num_players = env::args().nth(1).unwrap().parse().unwrap();
    let num_marbles = env::args().nth(2).unwrap().parse().unwrap();
    let score = solve(num_players, num_marbles);
    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_input() {
        assert_eq!(solve(455, 71223), 384288);
    }
}
