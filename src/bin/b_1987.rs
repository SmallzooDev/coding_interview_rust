// Baekjoon - 1987
// https://www.acmicpc.net/problem/1987

use std::{
    io::{self, Read, Write},
};
const DY: [i32; 4] = [-1, 0, 1, 0];
const DX: [i32; 4] = [0, 1, 0, -1];

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let mut board: Vec<Vec<char>> = vec![];

    let (y, _x): (usize, usize) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    for _i in 0..y {
        let tmp = lines.next().unwrap().chars().collect();
        board.push(tmp);
    }

    let mut visited = [false; 26];
    visited[(board[0][0] as u8 - b'A') as usize] = true;
    let output = dfs(board.as_slice(), &mut visited, 0, 0, 1);

    write!(stdout, "{}", output).unwrap();
}

fn dfs(board: &[Vec<char>], visited: &mut [bool; 26], y: usize, x: usize, current_length: usize) -> usize {
    let mut max_path = current_length;
    let max_y = board.len() as i32;
    let max_x = board[0].len() as i32;

    for i in 0..4 {
        let ty = y as i32 + DY[i];
        let tx = x as i32 + DX[i];

        if ty < 0 || ty >= max_y || tx < 0 || tx >= max_x {
            continue;
        }

        let ny = ty as usize;
        let nx = tx as usize;
        let next_char = board[ny][nx];
        let idx = (next_char as u8 - b'A') as usize;

        if visited[idx] {
            continue;
        }

        visited[idx] = true;
        let length = dfs(board, visited, ny, nx, current_length + 1);
        max_path = max_path.max(length);
        visited[idx] = false;
    }

    max_path
}
