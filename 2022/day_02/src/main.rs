use std::collections::HashSet;
use std::io::{stdin, Read};

fn find_common(strings: &Vec<&str>) -> char {
    let char_sets: Vec<HashSet<char>> = strings.into_iter().map(|x| x.chars().collect()).collect();

    let mut intersection = &char_sets[0];
    let mut temp = HashSet::new();

    for set in &char_sets[1..] {
        temp = intersection.intersection(set).cloned().collect();
        intersection = &temp;
    }
    let vec: Vec<&char> = intersection.into_iter().collect();
    *vec[0]
}

fn get_score(x: char) -> u32 {
    match x {
        x if x.is_uppercase() => x as u32 - 'A' as u32 + 27,
        x => x as u32 - 'a' as u32 + 1,
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: Vec<&str> = input.lines().collect();

    let result_0: u32 = input
        .clone()
        .into_iter()
        .map(|line| {
            let common = find_common(&vec![
                &line[0..line.len() / 2],
                &line[line.len() / 2..line.len()],
            ]);
            get_score(common)
        })
        .sum();

    let mut result_1 = 0;
    let mut chunk = vec![];
    let mut i = 0;
    while i < input.len() {
        chunk.push(input[i]);
        i += 1;
        if chunk.len() < 3 {
            continue
        } else {
            result_1 += get_score(find_common(&chunk));
            chunk.clear();
        }
    }

    println!("part 1: {:?}", result_0);
    println!("part 2: {:?}", result_1);
}
