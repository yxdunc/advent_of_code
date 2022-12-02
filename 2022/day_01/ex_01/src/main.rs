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
            (left, 2) => {WINNING_SCORE + ((left + 1) % 3) + 1},
            (left, 1) => {DRAW_SCORE + left + 1},
            (left, 0) => {LOSING_SCORE + ((left + 2) % 3) + 1}
            _ => { 0 }
        }
    }).sum();

    println!("{:?}", result);
}