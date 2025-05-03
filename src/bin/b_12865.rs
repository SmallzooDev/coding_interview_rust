// Baekjoon - 12865
// https://www.acmicpc.net/problem/12865
use std::cmp;
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let mut iter = lines.next().unwrap().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let vals: Vec<(usize, usize)> = lines
        .take(n)
        .map(|line| {
            let mut nums = line.split_whitespace();
            let weight = nums.next().unwrap().parse().unwrap();
            let value = nums.next().unwrap().parse().unwrap();
            (weight, value)
        })
        .collect();

    let mut dp = vec![vec![0; k + 1]; n + 1];

    for i in 1..=n {
        for w in 1..=k {
            let (item_weight, item_value) = vals[i - 1];

            if item_weight <= w {
                dp[i][w] = cmp::max(dp[i - 1][w], dp[i - 1][w - item_weight] + item_value);
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }

    write!(stdout, "{}", dp[n][k]).unwrap();
}
