// Baekjoon - 3085
// https://www.acmicpc.net/problem/3085

use std::cmp::max;
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut board: Vec<Vec<char>> = vec![];

    for _i in 0..n {
        let tmp = lines.next().unwrap().chars().collect();
        board.push(tmp);
    }

    let mut max_count = count_serial(&board, n);

    for i in 0..n {
        for j in 0..n-1 {
            board[i].swap(j, j + 1);
            max_count = max(max_count, count_serial(&board, n));
            board[i].swap(j, j + 1);
        }
    }

    for i in 0..n-1 {
        for j in 0.. n {
            let temp = board[i][j];
            board[i][j] = board[i + 1][j];
            board[i + 1][j] = temp;

            max_count = max(max_count, count_serial(&board, n));

            let temp = board[i][j];
            board[i][j] = board[i + 1][j];
            board[i + 1][j] = temp;
        }

    }

    write!(stdout, "{}", max_count).unwrap();
}

fn count_serial(inputs: &[Vec<char>], len: usize) -> i32 {
    let mut max_count = 0;

    for row in inputs.iter().take(len) {
        let mut current_count = 1;
        for j in 1..len {
            if j < row.len() && row[j] == row[j-1] {
                current_count += 1;
            } else {
                max_count = max(max_count, current_count);
                current_count = 1;
            }
        }
        max_count = max(max_count, current_count);
    }

    for j in 0..len {
        let mut current_count = 1;
        for i in 1..len {
            if i < inputs.len() && j < inputs[i].len() && j < inputs[i-1].len() && inputs[i][j] == inputs[i-1][j] {
                current_count += 1;
            } else {
                max_count = max(max_count, current_count);
                current_count = 1;
            }
        }
        max_count = max(max_count, current_count);
    }

    max_count
}

