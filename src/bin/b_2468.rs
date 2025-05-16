// Baekjoon - 2468
// https://www.acmicpc.net/problem/2468

use std::cmp::max;
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut water_max: usize = 0;
    let mut graph: Vec<Vec<usize>> = vec![];

    for i in 0..n {
        let tmp: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let tmp_max = tmp.iter().max().unwrap();
        water_max = max(water_max, *tmp_max);
        graph.push(tmp);
    }

    let mut output: usize = 1;

    for s in 0..water_max {
        let mut rain_dropped_graph = rain_drop(n, &graph, s);
        let mut count: usize = 0;
        for i in 0..n {
            for j in 0..n {
                if rain_dropped_graph[i][j] == 1 {
                    dfs(&mut rain_dropped_graph, i, j);
                    count = count + 1;
                }
            }
        }

        output = max(count, output);
    }

    write!(stdout, "{}", output).unwrap();
}

fn rain_drop(size: usize, graph: &[Vec<usize>], height: usize) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = vec![];
    for i in 0..size {
        let mut tmp = vec![0; size];
        for j in 0..size {
            if graph[i][j] > height {
                tmp[j] = 1;
            }
        }
        result.push(tmp);
    }
    result
}


fn dfs(graph: &mut Vec<Vec<usize>>, y: usize, x: usize) {
    graph[y][x] = 0;

    let dy: [i32; 4] = [1, 0, -1, 0];
    let dx: [i32; 4] = [0, 1, 0, -1];

    for i in 0..4 {
        let ny = y as i32 + dy[i];
        let nx = x as i32 + dx[i];

        if ny >= 0 && ny < graph.len() as i32 && nx >= 0 && nx < graph[0].len() as i32 {
            let ny = ny as usize;
            let nx = nx as usize;

            if graph[ny][nx] == 1 {
                dfs(graph, ny, nx);
            }
        }
    }
}