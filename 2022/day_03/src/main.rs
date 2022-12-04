use std::io::{stdin, Read};
use std::ops::Range;

fn is_contained(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    r1.start >= r2.start && r1.end <= r2.end
}
fn is_overlaping(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    r1.start <= r2.end && r1.end >= r2.start
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: Vec<&str> = input.lines().collect();

    let result_0 = input
        .clone()
        .into_iter()
        .map(|x| {
            x.split(",")
                .map(|x| x.split("-").map(|x| x.parse::<i32>().unwrap()).collect()).collect()
        })
        .filter(|x: &Vec<Vec<i32>>| {
            let r1 = x[0][0]..x[0][1];
            let r2 = x[1][0]..x[1][1];
            is_contained(&r1, &r2) || is_contained(&r2, &r1)
        })
        .collect::<Vec<Vec<Vec<i32>>>>()
        .len();
    let result_1 = input
        .clone()
        .into_iter()
        .map(|x| {
            x.split(",")
                .map(|x| x.split("-").map(|x| x.parse::<i32>().unwrap()).collect()).collect()
        })
        .filter(|x: &Vec<Vec<i32>>| {
            let r1 = x[0][0]..x[0][1];
            let r2 = x[1][0]..x[1][1];
            is_overlaping(&r1, &r2)
        })
        .collect::<Vec<Vec<Vec<i32>>>>()
        .len();

    println!("part 1: {:?}", result_0);
    println!("part 2: {:?}", result_1);
}
