// Baekjoon - 2225
// https://www.acmicpc.net/problem/2225

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap().split_whitespace().collect::<Vec<&str>>();
    let n: usize = first_line[0].parse().unwrap();
    let k: usize = first_line[1].parse().unwrap();

    // dp[i][j] 는 합이 i가 되도록 j개의 0 이상 정수 사용하는 경우의 수
    let mut dp = vec![vec![0i64; k + 1]; n + 1];

    dp[0][0] = 1;

    for j in 1..=k {
        for i in 0..=n {
            // 마지막에 더하는 수가 x라면, 나머지 j-1개의 수로 i-x를 만들어야 한다.
            for x in 0..=i {
                dp[i][j] = (dp[i][j] + dp[i - x][j - 1]) % 1000000000;
            }
        }
    }

    let output = dp[n][k];
    write!(stdout, "{}", output).unwrap();
}
