// Baekjoon - 2133
// https://www.acmicpc.net/problem/2133

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    if n % 2 != 0 {
        writeln!(stdout, "0").unwrap();
        return;
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[2] = 3;

    for i in (4..=n).step_by(2) {
        dp[i] = dp[i - 2] * 3;
        for j in (0..=i - 4).step_by(2) {
            dp[i] += dp[j] * 2;
        }
    }

    write!(stdout, "{}", dp[n]).unwrap();
}
