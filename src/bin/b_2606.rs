// Baekjoon - 2606
// https://www.acmicpc.net/problem/2606
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let m: usize = lines.next().unwrap().parse().unwrap();
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 0..m {
        let tmp: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let tn = tmp[0];
        let tv = tmp[1];
        graph[tn].push(tv);
        graph[tv].push(tn);
    }

    let mut visited = vec![false; n + 1];
    dfs(&graph, &mut visited, 1);

    // 시작 컴퓨터(1번) 제외
    let output = visited.iter().skip(2).filter(|&&v| v).count();
    writeln!(stdout, "{}", output).unwrap();
}

fn dfs(graph: &[Vec<usize>], visited: &mut [bool], start: usize) {
    visited[start] = true;
    for &i in &graph[start] {
        if !visited[i] {
            dfs(graph, visited, i);
        }
    }
}
