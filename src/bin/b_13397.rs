// Baekjoon - 13397
// https://www.acmicpc.net/problem/13397

use std::cmp::{max, min};
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

    let _n = meta_line[0];
    let m = meta_line[1];

    let nums: Vec<usize> = lines.next().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    if nums.iter().min() == nums.iter().max() {
        writeln!(stdout, "0").unwrap();
        return;
    }

    let mut left: usize = 0;
    let mut right = *nums.iter().max().unwrap();
    let mut output = right;

    while left < right {
        let mid = (left + right) / 2;

        if can_split(&nums, m, mid) {
            output = mid;
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    write!(stdout, "{}", output).unwrap();
}

fn can_split(nums: &[usize], m: usize, max_score: usize) -> bool {
    let mut count: usize = 1;

    let mut count_min = nums[0];
    let mut count_max = nums[0];

    for i in 1..nums.len() {
        let tmp_min = min(count_min, nums[i]);
        let tmp_max = max(count_max, nums[i]);

        if tmp_max - tmp_min > max_score {
            count += 1;
            count_min = nums[i];
            count_max = nums[i];
        } else {
            count_min = tmp_min;
            count_max = tmp_max;
        }
    }

    count <= m
}