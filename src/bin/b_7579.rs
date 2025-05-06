// Baekjoon - 7579
// https://www.acmicpc.net/problem/7579

use std::cmp::{max, min};
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let parts: Vec<usize> = lines.next().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (n, target_bytes) = (parts[0], parts[1]);

    let input_bytes: Vec<usize> = lines.next().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let input_restarts: Vec<usize> = lines.next().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let max_cost: usize = input_restarts.iter().sum();
    let mut dp = vec![0; max_cost + 1];

    for i in 0..n {
        let mem = input_bytes[i];
        let cost = input_restarts[i];

        for j in (cost..=max_cost).rev() {
            dp[j] = max(dp[j], dp[j - cost] + mem);
        }
    }

    let mut answer = usize::MAX;
    for cost in 0..=max_cost {
        if dp[cost] >= target_bytes {
            answer = min(answer, cost);
        }
    }

    write!(stdout, "{}", answer).unwrap();
}

// fn get_minimum_bytes_to_restart(
//     inputs: &[(usize, usize)],
//     target_bytes: usize,
//     tmp: &mut Vec<(usize, usize)>,
//     idx: usize,
// ) -> usize {
//     let (sum_bytes, sum_costs): (usize, usize) = tmp.iter()
//         .fold((0, 0), |(a, b), (x, y)| (a + x, b + y));
//
//     if sum_bytes >= target_bytes {
//         return sum_costs;
//     }
//
//     let mut result = usize::MAX;
//
//     for i in idx..inputs.len() {
//         tmp.push(inputs[i]);
//         result = min(result, get_minimum_bytes_to_restart(inputs, target_bytes, tmp, i + 1));
//         tmp.pop();
//     }
//
//     result
// }
