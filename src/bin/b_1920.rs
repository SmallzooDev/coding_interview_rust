// Baekjoon - 1920
// https://www.acmicpc.net/problem/1920

use std::collections::HashSet;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    line.clear();
    stdin.read_line(&mut line).unwrap();
    let nums: HashSet<i32> = line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    line.clear();
    stdin.read_line(&mut line).unwrap();

    line.clear();
    stdin.read_line(&mut line).unwrap();

    let mut result = String::new();
    for num in line.split_whitespace() {
        let target: i32 = num.parse().unwrap();
        if nums.contains(&target) {
            result.push_str("1\n");
        } else {
            result.push_str("0\n");
        }
    }

    write!(stdout, "{}", result).unwrap();
}

// fn binary_search(arr: &[i32], target: i32) -> bool {
//     let mut left = 0;
//     let mut right = arr.len() - 1;
//
//     while left <= right {
//         let mid = left + (right - left) / 2;
//         if arr[mid] == target {
//             return true;
//         }
//
//         if arr[mid] < target {
//             left = mid + 1;
//         } else {
//             if mid == 0 {
//                 break;
//             }
//             right = mid - 1;
//         }
//     }
//
//     false
// }
