// Baekjoon - 11650
// https://www.acmicpc.net/problem/11650

use std::io::{self, Read, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut vec: Vec<(i32, i32)> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let first: i32 = parts.next().unwrap().parse().unwrap();
        let second: i32 = parts.next().unwrap().parse().unwrap();
        vec.push((first, second));
    }

    vec.sort();

    let mut output = String::with_capacity(n * 10);
    for (first, second) in vec {
        output.push_str(&format!("{} {}\n", first, second));
    }

    write!(stdout, "{}", output).unwrap();
}
