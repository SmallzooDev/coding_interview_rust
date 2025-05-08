// Baekjoon - 3020
// https://www.acmicpc.net/problem/3020
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = parts[0];
    let h = parts[1];

    let mut bottom = Vec::with_capacity(n / 2);
    let mut top = Vec::with_capacity(n / 2);

    for i in 0..n {
        let height: usize = lines.next().unwrap().parse().unwrap();

        if i % 2 == 0 {
            bottom.push(height);
        } else {
            top.push(height);
        }
    }

    bottom.sort_unstable();
    top.sort_unstable();

    let mut min_obstacles = n;
    let mut count = 0;

    for fly_height in 1..=h {
        let bottom_hits = bottom.len() - lower_bound(&bottom, fly_height);
        let top_hits = top.len() - lower_bound(&top, h - fly_height + 1);
        let total_hits = bottom_hits + top_hits;

        if total_hits < min_obstacles {
            min_obstacles = total_hits;
            count = 1;
        } else if total_hits == min_obstacles {
            count += 1;
        }
    }

    writeln!(stdout, "{} {}", min_obstacles, count).unwrap();
}

fn lower_bound(arr: &[usize], target: usize) -> usize {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}
