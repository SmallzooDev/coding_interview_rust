// Baekjoon - 1912
// https://www.acmicpc.net/problem/1912

use std::cmp::max;
use std::io::{self, Read, Write};


fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let _n: usize = lines.next().unwrap().parse().unwrap();
    let mut inputs: Vec<i32> = lines.next().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // 지금까지 발견된 최대 부분합
    let mut max_so_far = inputs[0];
    // 현재 위치에서 끝나는 최대 부분합
    let mut max_ending_here = inputs[0];

    for i in 1..inputs.len() {
        max_ending_here = max(inputs[i], max_ending_here + inputs[i]);
        max_so_far = max(max_so_far, max_ending_here);
    }

    write!(stdout, "{}", max_so_far).unwrap();

}

// fn main() {
//     let mut stdin = io::stdin().lock();
//     let mut stdout = io::stdout().lock();
//     let mut input = String::new();
//     stdin.read_to_string(&mut input).unwrap();
//     let mut lines = input.lines();
//
//     let n: usize = lines.next().unwrap().parse().unwrap();
//     let mut inputs: Vec<i32> = lines.next().unwrap()
//         .split_whitespace()
//         .map(|x| x.parse().unwrap())
//         .collect();
//
//     let mut p_sum = vec![0; inputs.len()];
//     let mut dp = vec![0; inputs.len()];
//
//     p_sum[0] = inputs[0];
//     dp[0] = inputs[0];
//
//     for i in 1..inputs.len() {
//         p_sum[i] = p_sum[i - 1] + inputs[i];
//     }
//
//     for i in 1..inputs.len() {
//         let mut tmp_max = p_sum[i];
//         for j in 0..i {
//            tmp_max = max(p_sum[i] - p_sum[j], tmp_max)
//         }
//         dp[i] = tmp_max;
//     }
//
//     let output = dp.iter().max().unwrap();
//     write!(stdout, "{}", output).unwrap();
// }
