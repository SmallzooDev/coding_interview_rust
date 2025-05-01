// Baekjoon - 1342
// https://www.acmicpc.net/problem/1342
use std::{
    collections::HashMap,
    io::{self, Read, Write},
};

fn get_happy_num_count(
    len: usize,
    tmp_len: usize,
    last_char: Option<usize>,
    counts: &mut [i32; 26],
    memo: &mut HashMap<(usize, Option<usize>, [i32; 26]), i32>,
) -> i32 {
    if tmp_len == len {
        return 1;
    }

    let key = (tmp_len, last_char, *counts);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut result = 0;

    for i in 0..26 {
        if counts[i] > 0 && Some(i) != last_char {
            counts[i] -= 1;
            result += get_happy_num_count(len, tmp_len + 1, Some(i), counts, memo);
            counts[i] += 1;
        }
    }

    memo.insert(key, result);
    result
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let input_line = input.lines().next().unwrap().trim();
    let len = input_line.len();
    let mut counts = [0i32; 26];

    for c in input_line.chars() {
        counts[c as usize - 'a' as usize] += 1;
    }

    let mut memo = HashMap::new();
    let count = get_happy_num_count(len, 0, None, &mut counts, &mut memo);

    writeln!(stdout, "{}", count).unwrap();
}
