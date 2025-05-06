// Baekjoon - 11054
// https://www.acmicpc.net/problem/11054

use std::cmp::max;
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let inputs: Vec<usize> = lines.next().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut dp_inc = vec![1; n];
    let mut dp_dec = vec![1; n];

    for i in 0..n {
        for j in 0..i {
            if inputs[i] > inputs[j] {
                dp_inc[i] = max(dp_inc[i], dp_inc[j] + 1)
            }
        }
    }

    for i in (0..n).rev() {
        for j in (i + 1)..n {
            if inputs[i] > inputs[j] {
                dp_dec[i] = max(dp_dec[i], dp_dec[j] + 1)
            }
        }
    }

    let mut output = 0;
    for i in 0..n {
        output = output.max(dp_inc[i] + dp_dec[i] - 1)
    }

    write!(stdout, "{}", output).unwrap();
}
