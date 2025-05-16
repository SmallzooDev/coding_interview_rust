// Baekjoon - 1697
// https://www.acmicpc.net/problem/1697

use std::collections::VecDeque;
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // let n: usize = lines.next().unwrap().parse().unwrap();
    let first_line: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = first_line[0];
    let k = first_line[1];

    let output = bfs(n, k);

    write!(stdout, "{}", output).unwrap();
}

fn bfs(n: usize, k: usize) -> i32 {
    let max_location_size = 100001;
    let mut visited = vec![-1; max_location_size];
    let mut queue = VecDeque::new();

    queue.push_back(n);
    visited[n] = 0;

    while let Some(current) = queue.pop_front() {
        if current == k {
            return visited[current];
        }

        let next_positions = [
            if current > 0 { Some(current - 1) } else { None },
            if current < max_location_size - 1 {
                Some(current + 1)
            } else {
                None
            },
            if current * 2 < max_location_size {
                Some(current * 2)
            } else {
                None
            },
        ];

        for &next in next_positions.iter().flatten() {
            if visited[next] == -1 {
                visited[next] = visited[current] + 1;
                queue.push_back(next);
            }
        }
    }

    -1
}
