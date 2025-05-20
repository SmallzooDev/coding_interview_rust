// Baekjoon - 2839
// https://www.acmicpc.net/problem/2839

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();
    let n_usize = n as usize;

    let mut dp: Vec<i32> = vec![i32::MAX - 1; (n_usize + 1)];

    dp[0] = 0;

    for i in 1..=n_usize {
        if i >= 3 && dp[i - 3] != i32::MAX - 1 {
            dp[i] = dp[i].min(dp[i - 3] + 1);
        }

        if i >= 5 && dp[i - 5] != i32::MAX - 1 {
            dp[i] = dp[i].min(dp[i - 5] + 1);
        }
    }

    if dp[n_usize] == i32::MAX - 1 {
        write!(stdout, "-1").unwrap();
    } else {
        write!(stdout, "{}", dp[n_usize]).unwrap();
    }
}