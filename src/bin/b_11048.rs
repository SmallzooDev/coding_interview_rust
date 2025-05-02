// Baekjoon - 11048
// https://www.acmicpc.net/problem/11048

use std::{
    cmp,
    io::{self, Read, Write},
};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let dimensions: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let y = dimensions[0];
    let x = dimensions[1];

    let mut candy = vec![vec![0; x + 1]; y + 1];
    let mut dp = vec![vec![0; x + 1]; y + 1];

    for (i, line) in lines.enumerate().take(y) {
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        for (j, &value) in row.iter().enumerate().take(x) {
            candy[i + 1][j + 1] = value;
        }
    }

    dp[1][1] = candy[1][1];

    for i in 1..=y {
        for j in 1..=x {
            if i == 1 && j == 1 {
                continue;
            }
            let from_up = if i > 1 { dp[i - 1][j] } else { 0 };
            let from_left = if j > 1 { dp[i][j - 1] } else { 0 };
            let from_diagonal = if i > 1 && j > 1 { dp[i - 1][j - 1] } else { 0 };

            dp[i][j] = candy[i][j] + cmp::max(from_up, cmp::max(from_left, from_diagonal));
        }
    }

    write!(stdout, "{}", dp[y][x]).unwrap();
}
