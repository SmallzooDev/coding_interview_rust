// Baekjoon - 1011
// https://www.acmicpc.net/problem/1011

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let targets: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let tmp_distance = targets[1] - targets[0];
        let output = get_distance(tmp_distance);
        writeln!(stdout, "{}", output).unwrap();
    }
}

fn get_distance(distance: usize) -> usize {
    let mut n: usize = 1;
    while n * n <= distance {
        n += 1;
    }
    n -= 1;

    if distance <= n * n {
        2 * n - 1
    } else if distance <= n * n + n {
        2 * n
    } else {
        2 * n + 1
    }
}
