// Baekjoon - 2110
// https://www.acmicpc.net/problem/2110

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();

    let parts: Vec<u64> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = parts[0];
    let c = parts[1];

    let mut houses: Vec<u64> = vec![];

    for i in 0..n {
        let tmp: u64 = lines.next().unwrap().parse().unwrap();
        houses.push(tmp);
    }

    houses.sort_unstable();

    let mut left = 1;
    let mut right = houses[houses.len() - 1] - houses[0];
    let mut result = 0;

    while left <= right {
        let mid = (left + right) / 2;

        if can_install(mid, &houses, c) {
            result = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }


    write!(stdout, "{}", result).unwrap();
}

fn can_install(distance: u64, houses: &[u64], c: u64) -> bool {
    let mut count = 1;
    let mut last_position = houses[0];

    for &house in houses.iter().skip(1) {
        if house - last_position >= distance {
            count += 1;
            last_position = house;

            if count >= c {
                return true;
            }
        }
    }

    false
}