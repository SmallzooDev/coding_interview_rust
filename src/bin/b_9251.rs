// Baekjoon - 9251
// https://www.acmicpc.net/problem/9251

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_str: Vec<char> = lines.next().unwrap().chars().collect();
    let second_str: Vec<char> = lines.next().unwrap().chars().collect();

    let m = first_str.len();
    let n = second_str.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if first_str[i - 1] == second_str[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    write!(stdout, "{}", dp[m][n]).unwrap();
}
