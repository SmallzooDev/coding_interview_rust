// Baekjoon - 1660
// https://www.acmicpc.net/problem/1660

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut piles = Vec::new();
    let mut triangle = 1;
    let mut tetra = 1;

    while tetra <= n {
        piles.push(tetra);
        let next_level = piles.len() + 1;
        triangle = next_level * (next_level + 1) / 2;
        tetra += triangle;
    }

    let mut dp = vec![usize::MAX - 1; n + 1];
    dp[0] = 0;

    for i in 0..=n {
        for &pile in &piles {
            if i >= pile {
                dp[i] = dp[i].min(dp[i - pile] + 1);
            }
        }
    }

    write!(stdout, "{}", dp[n]).unwrap();
}
