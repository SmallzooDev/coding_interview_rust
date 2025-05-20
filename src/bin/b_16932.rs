// Baekjoon - 16932
// https://www.acmicpc.net/problem/16932

use std::collections::{HashSet, VecDeque};
use std::io;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let max_y = first_line[0];
    let max_x = first_line[1];
    let mut grid: Vec<Vec<usize>> = vec![];
    let mut targets: Vec<(usize, usize)> = vec![];

    for i in 0..max_y {
        let tmp: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        for j in 0..tmp.len() {
            if tmp[j] == 0 {
                targets.push((i, j));
            }
        }

        grid.push(tmp);
    }
    let mut group_id = 2;
    let mut group_sizes: Vec<usize> = vec![0];
    group_sizes.push(0);
    let mut visited = vec![vec![false; max_x]; max_y];

    for i in 0..max_y {
        for j in 0..max_x {
            if grid[i][j] == 1 && !visited[i][j] {
                let size = bfs(&mut grid, &mut visited, i, j, group_id, max_y, max_x);
                group_sizes.push(size);
                group_id += 1;
            }
        }
    }

    let mut max_size = 0;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for &(y, x) in &targets {
        let mut adj = HashSet::new();

        for &(dy, dx) in &directions {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny >= 0 && ny < max_y as isize && nx >= 0 && nx < max_x as isize {
                let ny = ny as usize;
                let nx = nx as usize;

                if grid[ny][nx] >= 2 {
                    adj.insert(grid[ny][nx]);
                }
            }
        }

        let mut total_size = 1;
        for &group in &adj {
            total_size += group_sizes[group];
        }

        max_size = max_size.max(total_size);
    }

    write!(stdout, "{}", max_size).unwrap();
}

fn bfs(
    grid: &mut [Vec<usize>],
    visited: &mut [Vec<bool>],
    start_y: usize,
    start_x: usize,
    group_id: usize,
    max_y: usize,
    max_x: usize,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start_y, start_x));
    visited[start_y][start_x] = true;
    grid[start_y][start_x] = group_id;

    let mut size = 1;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((y, x)) = queue.pop_front() {
        for &(dy, dx) in &directions {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny >= 0 && ny < max_y as isize && nx >= 0 && nx < max_x as isize {
                let ny = ny as usize;
                let nx = nx as usize;

                if grid[ny][nx] == 1 && !visited[ny][nx] {
                    visited[ny][nx] = true;
                    grid[ny][nx] = group_id;
                    queue.push_back((ny, nx));
                    size += 1;
                }
            }
        }
    }

    size
}