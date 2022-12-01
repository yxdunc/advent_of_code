use std::cmp::min;
use std::fmt::Debug;
use std::io::{Read, stdin};



/// O(n * len(list))
/// best for small value of n
fn get_max_n<T: Ord + Debug>(list: &Vec<T>, n: usize) -> Vec<&T> {
    let mut top_n: Vec<&T>  = vec![];
    let mut limit: Option<&T> = None;
    let mut i = 0;

    while i < n && top_n.len() < n {
        let mut max_y = list.iter().min().unwrap();
        let mut qt_max_y= 1;
        for y in list {
            if let Some(limit) = limit {
                if y >= limit {
                    continue;
                }
            }
            match y {
                y if y == max_y => {qt_max_y += 1}
                y if y > max_y => {max_y = &y; qt_max_y = 1}
                _ => {}
            }
        }
        top_n.append(&mut vec![max_y; qt_max_y]);
        limit = Some(max_y);
        i += 1;
    }

    top_n[0..min(n, top_n.len())].to_owned()
}


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

    // trivial solution
    // group_sums.sort_by(|a, b| b.cmp(a));
    // let top_3_sum: i32 = group_sums[0..3].iter().sum();

    // optimized for small n (n=3)
    let top_3_sum = get_max_n(&group_sums, 3).iter().map(|x| **x).sum::<i32>();

    println!("{:?}", top_3_sum);
}