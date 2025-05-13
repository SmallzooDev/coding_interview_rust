// Baekjoon - 17609
// https://www.acmicpc.net/problem/17609

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let tmp = lines.next().unwrap();
        let result = is_palindrome(tmp);
        writeln!(stdout, "{}", result).unwrap();
    }
}

fn is_palindrome(case: &str) -> usize {
    let chars: Vec<char> = case.chars().collect();
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        if chars[left] != chars[right] {
            let skip_left = is_sub_palindrome(&chars, left + 1, right);

            let skip_right = is_sub_palindrome(&chars, left, right - 1);

            return if skip_left || skip_right {
                1
            } else {
                2
            };
        }

        left += 1;
        right -= 1;
    }
    0
}

fn is_sub_palindrome(chars: &[char], mut left: usize, mut right: usize) -> bool {
    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
