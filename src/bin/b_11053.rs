// Baekjoon - 11053
// https://www.acmicpc.net/problem/11053

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _n: usize = lines.next().unwrap().parse().unwrap();
    let numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![1; numbers.len()];

    for i in 1..numbers.len() {
        for j in 0..i {
            if numbers[j] < numbers[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }

    let max_len = dp.iter().max().unwrap();
    write!(stdout, "{}", max_len).unwrap();
}
