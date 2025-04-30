// Baekjoon - 1182
// https://www.acmicpc.net/problem/1182

use std::io::{self, Read, Write};

fn combination(
    vec: &Vec<i32>,
    selected: &mut Vec<i32>,
    idx: usize,
    level: usize,
    sum: i32,
    count: &mut i32,
) {
    if selected.len() == level {
        let total: i32 = selected.iter().sum();
        if total == sum {
            *count += 1;
        }
        return;
    }

    for i in idx..vec.len() {
        selected.push(vec[i]);
        combination(vec, selected, i + 1, level, sum, count);
        selected.pop();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let cnt: usize = parts.next().unwrap().parse().unwrap();
    let sum: i32 = parts.next().unwrap().parse().unwrap();

    let numbers_line = lines.next().unwrap();
    let vec: Vec<i32> = numbers_line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut total_count = 0;

    for level in 1..=cnt {
        let mut selected = Vec::new();
        combination(&vec, &mut selected, 0, level, sum, &mut total_count);
    }

    write!(stdout, "{}", total_count).unwrap();
}
