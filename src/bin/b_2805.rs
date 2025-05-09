// Baekjoon - 2805
// https://www.acmicpc.net/problem/2805

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let parts: Vec<u64> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let _n = parts[0];
    let need = parts[1];

    let trees: Vec<u64> = lines.next().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut left = 0;
    let mut right = *trees.iter().max().unwrap();
    let mut result = 0;

    while left <= right {
        let mid = (left + right) / 2;

        let amount: u64 = trees.iter()
            .map(|&height| if height > mid { height - mid } else { 0 })
            .sum();

        if amount >= need {
            result = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    write!(stdout, "{}", result).unwrap();
}
