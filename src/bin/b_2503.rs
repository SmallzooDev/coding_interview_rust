// Baekjoon - 2503
// https://www.acmicpc.net/problem/2503

use std::io::{self, Read, Write};

fn is_valid(candidate: &str, question: &str, strike: i32, ball: i32) -> bool {
    let mut s = 0;
    let mut b = 0;

    for (i, c) in candidate.chars().enumerate() {
        for (j, q) in question.chars().enumerate() {
            if c == q {
                if i == j {
                    s += 1;
                } else {
                    b += 1;
                }
            }
        }
    }

    s == strike && b == ball
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut vec: Vec<(&str, i32, i32)> = Vec::with_capacity(n);

    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let first: &str = parts.next().unwrap();
        let second: i32 = parts.next().unwrap().parse().unwrap();
        let third: i32 = parts.next().unwrap().parse().unwrap();

        vec.push((first, second, third));
    }

    let mut count = 0;

    for i in 123..988 {
        let candidate = i.to_string();

        let digits: Vec<char> = candidate.chars().collect();
        if digits.contains(&'0')
            || digits[0] == digits[1]
            || digits[1] == digits[2]
            || digits[0] == digits[2]
        {
            continue;
        }

        if vec.iter().all(|(q, s, b)| is_valid(&candidate, q, *s, *b)) {
            count += 1;
        }
    }

    write!(stdout, "{count}").unwrap();
}
