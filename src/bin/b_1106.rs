// Baekjoon - 1106
// https://www.acmicpc.net/problem/1106

use std::cmp::min;
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let mut costs: Vec<(usize, usize)> = vec![];
    let first_line: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let target = first_line[0];
    let n = first_line[1];

    for _ in 0..n {
        let tmp: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        costs.push((tmp[0], tmp[1]));
    }

    let max_target = target + 101;
    let mut dp = vec![usize::MAX; max_target + 1];
    dp[0] = 0;
    for i in 0..(max_target) {
        if dp[i] == usize::MAX {
            continue;
        }

        for &(cost, customers) in &costs {
            let next = i + customers;
            if next <= max_target {
                dp[next] = min(dp[next], dp[i] + cost);
            }
        }
    }

    let output = dp[target..=max_target].iter().min().unwrap();
    write!(stdout, "{}", output).unwrap();
}
