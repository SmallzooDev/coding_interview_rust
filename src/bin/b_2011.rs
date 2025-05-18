// Baekjoon - 2011
// https://www.acmicpc.net/problem/2011

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n = lines.next().unwrap();
    let chars: Vec<char> = n.chars().collect();

    if chars.is_empty() {
        write!(stdout, "0").unwrap();
        return;
    }

    if chars[0] == '0' {
        write!(stdout, "0").unwrap();
        return;
    }

    let mut dp: Vec<usize> = vec![0; chars.len() + 1];
    dp[0] = 1;

    dp[1] = 1;

    for i in 1..chars.len() {
        let current = chars[i] as u8 - b'0';
        if current > 0 {
            dp[i + 1] += dp[i];
        }

        let prev = chars[i - 1] as u8 - b'0';
        let two_digit = prev * 10 + current;
        if prev > 0 && two_digit >= 10 && two_digit <= 26 {
            dp[i + 1] += dp[i - 1];
        }

        if dp[i + 1] == 0 {
            write!(stdout, "0").unwrap();
            return;
        }

        dp[i + 1] %= 1000000;
    }

    write!(stdout, "{}", dp[chars.len()]).unwrap();
}
