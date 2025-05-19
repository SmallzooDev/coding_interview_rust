// Baekjoon - 12869
// https://www.acmicpc.net/problem/12869

use std::collections::VecDeque;
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let scv_hp: Vec<i32> = lines.next().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut hp = [0; 3];
    for i in 0..n {
        hp[i] = scv_hp[i];
    }

    let damage = [
        [9, 3, 1],
        [9, 1, 3],
        [3, 9, 1],
        [3, 1, 9],
        [1, 9, 3],
        [1, 3, 9]
    ];

    let mut visited = vec![vec![vec![-1; 61]; 61]; 61];

    let mut queue = VecDeque::new();
    queue.push_back((hp[0], hp[1], hp[2], 0));
    visited[hp[0] as usize][hp[1] as usize][hp[2] as usize] = 0;

    while let Some((h1, h2, h3, attacks)) = queue.pop_front() {
        if h1 <= 0 && h2 <= 0 && h3 <= 0 {
            write!(stdout, "{}", attacks).unwrap();
            return;
        }

        for dmg in &damage {
            let new_h1 = std::cmp::max(0, h1 - dmg[0]);
            let new_h2 = std::cmp::max(0, h2 - dmg[1]);
            let new_h3 = std::cmp::max(0, h3 - dmg[2]);

            if visited[new_h1 as usize][new_h2 as usize][new_h3 as usize] != -1 {
                continue;
            }

            visited[new_h1 as usize][new_h2 as usize][new_h3 as usize] = attacks + 1;
            queue.push_back((new_h1, new_h2, new_h3, attacks + 1));
        }
    }

    write!(stdout, "E").unwrap();
}
