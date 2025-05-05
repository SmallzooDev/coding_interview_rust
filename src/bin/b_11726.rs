// Baekjoon - 11726
// https://www.acmicpc.net/problem/11726

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    if n == 1 {
        writeln!(stdout, "1").unwrap();
        return;
    }

    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    dp[2] = 2;

    for i in 3..=n {
        dp[i] = (dp[i - 1] + dp[i - 2]) % 10007;
    }

    writeln!(stdout, "{}", dp[n]).unwrap();
}