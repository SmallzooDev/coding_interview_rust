// Baekjoon - 2869
// https://www.acmicpc.net/problem/2869

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let values: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let a = values[0];
    let b = values[1];
    let v = values[2];

    let daily_progress = a - b;

    let days = if (v - a) % daily_progress == 0 {
        (v - a) / daily_progress + 1
    } else {
        (v - a) / daily_progress + 2
    };

    let result = if a >= v { 1 } else { days };

    write!(stdout, "{}", result).unwrap();
}