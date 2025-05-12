// Baekjoon - 2003
// https://www.acmicpc.net/problem/2003

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let meta_line: Vec<usize> = lines.next().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = meta_line[0];
    let m = meta_line[1];

    let nums: Vec<usize> = lines.next().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut count = 0;
    let mut sum = 0;
    let mut left = 0;

    for right in 0..n {
        sum += nums[right];

        while sum >= m && left <= right {
            if sum == m {
                count += 1;
            }
            sum -= nums[left];
            left += 1;
        }
    }

    write!(stdout, "{}", count).unwrap();
}
