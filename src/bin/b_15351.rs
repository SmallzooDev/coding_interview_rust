// Baekjoon - 15351
// https://www.acmicpc.net/problem/15351

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let word = line.trim();

        let sum: i64 = word
            .chars()
            .filter(|c| *c != ' ')
            .map(|c| (c as u8 - b'A' + 1) as i64)
            .sum();

        if sum == 100 {
            println!("PERFECT LIFE");
        } else {
            println!("{}", sum);
        }
    }
}
