// Baekjoon - 2178
// https://www.acmicpc.net/problem/2178

use std::collections::VecDeque;
use std::io::{self, Read, Write};

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

    let y = first_line[0];
    let x = first_line[1];

    let mut map: Vec<Vec<usize>> = vec![];

    for _ in 0..y {
        let line = lines.next().unwrap();
        let row: Vec<usize> = line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        map.push(row);
    }

    let output = bfs(&map, y, x);
    write!(stdout, "{}", output).unwrap();
}

fn bfs(graph: &[Vec<usize>], n: usize, m: usize) -> i32 {
    let dy = [-1, 0, 1, 0];
    let dx = [0, 1, 0, -1];

    let mut dist: Vec<Vec<i32>> = vec![vec![-1; m]; n];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    dist[0][0] = 1;
    queue.push_back((0, 0));

    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();

        if y == n - 1 && x == m - 1 {
            return dist[y][x];
        }

        for i in 0..4 {
            let ny = y as i32 + dy[i];
            let nx = x as i32 + dx[i];

            if ny >= 0 && ny < n as i32 && nx >= 0 && nx < m as i32 {
                let ny = ny as usize;
                let nx = nx as usize;

                if dist[ny][nx] == -1 && graph[ny][nx] == 1 {
                    dist[ny][nx] = dist[y][x] + 1;
                    queue.push_back((ny, nx));
                }
            }
        }
    }

    -1
}
