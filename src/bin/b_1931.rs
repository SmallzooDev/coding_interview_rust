// Baekjoon - 1931
// https://www.acmicpc.net/problem/1931

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut schedules: Vec<(usize, usize)> = vec![];

    for i in 0..n {
        let tmp: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        schedules.push((tmp[0], tmp[1]));
    }

    schedules.sort_by(|a, b| {
        a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))
    });

    let mut result: Vec<(usize, usize)> = vec![];
    result.push(schedules[0]);

    for i in 1..n {
        let tmp = schedules[i];
        let former = result.last().unwrap();

        if tmp.0 >= former.1 {
            result.push(tmp);
        }
    }

    let output = result.len();
    write!(stdout, "{}", output).unwrap();
}
