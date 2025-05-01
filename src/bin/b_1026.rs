// Baekjoon - 1026
// https://www.acmicpc.net/problem/1026

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let _n: usize = lines.next().unwrap().parse().unwrap();

    let mut arrays: Vec<Vec<i32>> = lines
        .take(2)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    arrays[0].sort();
    arrays[1].sort_by(|a, b| b.cmp(a));

    let result: i32 = arrays[0]
        .iter()
        .zip(arrays[1].iter())
        .map(|(a, b)| a * b)
        .sum();

    write!(stdout, "{}", result).unwrap();
}
