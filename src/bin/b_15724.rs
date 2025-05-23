// Baekjoon - 15724
// https://www.acmicpc.net/problem/15724

use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let first_line: Vec<usize> = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let max_y = first_line[0];
    let max_x = first_line[1];

    let mut graph: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];

    for i in 1..=max_y {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let row: Vec<usize> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for j in 1..=max_x {
            graph[i][j] = row[j - 1] + graph[i - 1][j] + graph[i][j - 1] - graph[i - 1][j - 1];
        }
    }

    line.clear();
    reader.read_line(&mut line).unwrap();
    let k: usize = line.trim().parse().unwrap();

    for _ in 0..k {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let tmp: Vec<usize> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let x1 = tmp[0];
        let y1 = tmp[1];
        let x2 = tmp[2];
        let y2 = tmp[3];

        let result = graph[x2][y2]
            - graph[x1 - 1][y2]
            - graph[x2][y1 - 1]
            + graph[x1 - 1][y1 - 1];

        writeln!(writer, "{}", result).unwrap();
    }
}
