// Baekjoon - 1806
// https://www.acmicpc.net/problem/1806

use std::cmp::min;
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let meta_line: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = meta_line[0];
    let s = meta_line[1];
    let nums: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    if n == 1 {
        if nums[0] >= s {
            write!(stdout, "1").unwrap();
            return;
        } else {
            write!(stdout, "0").unwrap();
            return;
        }
    }

    let mut min_length = usize::MAX;
    let mut left = 0;
    let mut right = 0;
    let mut current_sum = 0;

    while right < n {
        current_sum += nums[right];

        while current_sum >= s && left <= right {
            min_length = min(min_length, right - left + 1);
            current_sum -= nums[left];
            left += 1;
        }

        right += 1;
    }

    if min_length == usize::MAX {
        write!(stdout, "0").unwrap();
    } else {
        write!(stdout, "{}", min_length).unwrap();
    }
}