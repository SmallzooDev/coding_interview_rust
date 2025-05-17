// Baekjoon - 15486
// https://www.acmicpc.net/problem/15486

use std::cmp::max;
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut schedule: Vec<usize> = vec![0; n];
    let mut payments: Vec<usize> = vec![0; n];

    for i in 0..n {
        let tmp_line: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        schedule[i] = tmp_line[0];
        payments[i] = tmp_line[1];
    }

    let mut dp: Vec<usize> = vec![0; n + 1];
    let mut max_profit = 0;

    for i in 0..n {
        max_profit = max(max_profit, dp[i]);

        let end_day = i + schedule[i];
        if end_day <= n {
            dp[end_day] = max(dp[end_day], max_profit + payments[i]);
        }
    }

    let output = max(max_profit, dp[n]);
    write!(stdout, "{}", output).unwrap();
}
