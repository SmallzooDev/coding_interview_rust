// Baekjoon - 2580
// https://www.acmicpc.net/problem/2580

use std::io::{self, Read, Write};

const MAX_LEN: usize = 9;

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let mut sudoku_board: Vec<Vec<usize>> = vec![];
    let mut targets: Vec<(usize, usize)> = vec![];

    for i in 0..MAX_LEN {
        let tmp: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        for j in 0..tmp.len() {
            if tmp[j] == 0 {
                targets.push((i, j));
            }
        }

        sudoku_board.push(tmp);
    }

    if solve(&mut sudoku_board, &targets, 0) {
        for row in sudoku_board {
            let line = row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
            writeln!(stdout, "{}", line).unwrap();
        }
    }
}

fn solve(board: &mut [Vec<usize>], targets: &[(usize, usize)], idx: usize) -> bool {
    if idx == targets.len() {
        return true;
    }

    let (y, x) = targets[idx];
    let mut used = [false; MAX_LEN];
    check_horizon(board, &mut used, y);
    check_vertical(board, &mut used, x);
    check_inside(board, &mut used, (y, x));

    for i in 1..=MAX_LEN {
        if used[i - 1] {
            continue;
        }
        board[y][x] = i;
        if solve(board, targets, idx + 1) {
            return true;
        }
        board[y][x] = 0;
    }

    false
}

fn check_horizon(board: &[Vec<usize>], used: &mut [bool], y: usize) {
    for i in 0..MAX_LEN {
        let tmp = board[y][i];
        if tmp == 0 {
            continue;
        }
        used[tmp - 1] = true;
    }
}

fn check_vertical(board: &[Vec<usize>], used: &mut [bool], x: usize) {
    for i in 0..MAX_LEN {
        let tmp = board[i][x];
        if tmp == 0 {
            continue;
        }
        used[tmp - 1] = true;
    }
}
fn check_inside(board: &[Vec<usize>], used: &mut [bool], target: (usize, usize)) {
    let block_start_y = (target.0 / 3) * 3;
    let block_start_x = (target.1 / 3) * 3;

    for dy in 0..3 {
        for dx in 0..3 {
            let ny = block_start_y + dy;
            let nx = block_start_x + dx;
            let tmp = board[ny][nx];
            if tmp != 0 {
                used[tmp - 1] = true;
            }
        }
    }
}
