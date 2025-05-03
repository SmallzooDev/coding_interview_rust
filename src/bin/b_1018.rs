// Baekjoon - 1018
// https://www.acmicpc.net/problem/1018

use std::{
    cmp::min,
    io::{self, Read, Write},
};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let mut iter = lines.next().unwrap().split_whitespace();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let board: Vec<Vec<char>> = lines.take(m).map(|line| line.chars().collect()).collect();

    let mut min_repaints = 8 * 8;

    for start_row in 0..=m - 8 {
        for start_col in 0..=n - 8 {
            let start_white = count_repaints(&board, start_row, start_col, 'W');
            let start_black = count_repaints(&board, start_row, start_col, 'B');

            min_repaints = min(min_repaints, min(start_white, start_black));
        }
    }

    write!(stdout, "{}", min_repaints).unwrap();
}

fn count_repaints(
    board: &[Vec<char>],
    start_row: usize,
    start_col: usize,
    start_color: char,
) -> usize {
    let mut count = 0;
    let other_color = if start_color == 'W' { 'B' } else { 'W' };

    for i in 0..8 {
        for j in 0..8 {
            let expected_color = if (i + j) % 2 == 0 {
                start_color
            } else {
                other_color
            };
            if board[start_row + i][start_col + j] != expected_color {
                count += 1;
            }
        }
    }
    count
}
