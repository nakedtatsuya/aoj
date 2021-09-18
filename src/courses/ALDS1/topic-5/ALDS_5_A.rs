#![allow(non_snake_case)]

use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let m: Vec<usize> = (0..q).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut candidates = HashSet::new();
    for i in 0..1<<n {
        let mut sum = 0;
        for j in 0..n {
            if i>>j & 1 > 0 {
                sum += a[j];
            }
        }
        candidates.insert(sum);
    }

    let mut result = String::new();
    for i in 0..q {
        if candidates.contains(&m[i]) {
            result.push_str(&format!("{}\n", "yes"));
        } else {
            result.push_str(&format!("{}\n", "no"));
        }
    }
    result.trim().to_string()
}


