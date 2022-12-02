use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    const WINNING_SCORE: u32 = 6;
    const DRAW_SCORE: u32 = 3;
    const LOSING_SCORE: u32 = 0;

    let result: u32 = input.split("\n").map(|x| {
        let left = x.chars().nth(0).unwrap() as u32 - 'A' as u32;
        let right = x.chars().nth(2).unwrap() as u32 - 'X' as u32;

        match (left, right) {
            (left, right) if left == right => {DRAW_SCORE + right + 1},
            (0, 2) | (1, 0) | (2, 1) => {LOSING_SCORE + right + 1},
            (_, right) => {WINNING_SCORE + right + 1}
        }
    }).sum();

    println!("{:?}", result);
}