// Baekjoon - 1260
// https://www.acmicpc.net/problem/1260

use std::collections::VecDeque;
use std::io::{self, Read, StdoutLock, Write};

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

    let n = first_line[0];
    let m = first_line[1];
    let v = first_line[2];

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 0..m {
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        graph[edge[0]].push(edge[1]);
        graph[edge[1]].push(edge[0]);
    }

    for i in 0..=n {
        graph[i].sort();
    }

    let mut visited = vec![false; n + 1];
    dfs(&graph, &mut visited, v, n + 1, &mut stdout);
    writeln!(stdout).unwrap();

    bfs(&graph, v, n + 1, &mut stdout);
    writeln!(stdout).unwrap();
}

fn dfs(graph: &[Vec<usize>], visited: &mut [bool], start: usize, max: usize, stdout: &mut StdoutLock) {
    visited[start] = true;
    write!(stdout, "{} ", start).unwrap();

    for &next in &graph[start] {
        if !visited[next] {
            dfs(graph, visited, next, max, stdout);
        }
    }
}

fn bfs(graph: &[Vec<usize>], start: usize, max: usize, stdout: &mut StdoutLock) {
    let mut visited = vec![false; max];
    let mut queue = VecDeque::new();

    visited[start] = true;
    write!(stdout, "{} ", start).unwrap();
    queue.push_back(start);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        for &next in &graph[current] {
            if !visited[next] {
                visited[next] = true;
                write!(stdout, "{} ", next).unwrap();
                queue.push_back(next);
            }
        }
    }
}
