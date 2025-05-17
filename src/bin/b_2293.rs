// Baekjoon - 2293
// https://www.acmicpc.net/problem/2293

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = first_line[0];
    let k = first_line[1];

    let mut coins: Vec<usize> = vec![];
    for _ in 0..n {
        let tmp: usize = lines.next().unwrap().parse().unwrap();
        coins.push(tmp);
    }

    let mut dp: Vec<i32> = vec![0; k + 1];
    dp[0] = 1;

    for &coin in &coins {
        for i in coin..=k {
            dp[i] += dp[i - coin];
        }
    }

    let output = dp[k];
    write!(stdout, "{}", output).unwrap();
}
