use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();
    let lines = input.split("\n").map(|x| x.parse::<i32>().ok()).collect::<Vec<Option<i32>>>();

    let mut group_sums = vec![0];

    for x in lines {
        match x {
            None => { group_sums.push(0) }
            Some(x) => { *group_sums.last_mut().unwrap() += x }
        }
    }
    println!("{:?}", group_sums.iter().max().unwrap());
}