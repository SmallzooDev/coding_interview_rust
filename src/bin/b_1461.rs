// https://www.acmicpc.net/problem/1461
// Baekjoon - 1461

use std::{
    cmp,
    io::{self, Read, Write},
};

// -1,  3 (4 5) (6 11)
// (-45 -26 -18) (-9 -4),  (22 40 50)
fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let _n: usize = parts.next().unwrap().parse().unwrap();
    let holdable: usize = parts.next().unwrap().parse().unwrap();

    // 음수와 양수는 다르게 접근해야함
    let mut plus: Vec<i32> = Vec::new();
    let mut minus: Vec<i32> = Vec::new();

    if let Some(line) = lines.next() {
        for num_str in line.split_whitespace() {
            if let Ok(num) = num_str.parse::<i32>() {
                if num >= 0 {
                    plus.push(num);
                } else {
                    minus.push(num.abs());
                }
            }
        }
    }

    // 내림차순
    plus.sort_by(|a, b| b.cmp(a));
    minus.sort_by(|a, b| b.cmp(a));

    let mut result = 0;
    let mut max_dist = 0;

    // 내림차순, 0번원소부터 즉 가장 큰 원소부터 한번에 들 수 있는 값만큼 step_by()
    for i in (0..plus.len()).step_by(holdable) {
        // 가장 긴 거리를 카운트
        if i == 0 {
            max_dist = cmp::max(max_dist, plus[i]);
        }

        result += plus[i] * 2;
    }

    // 위와 동일
    for i in (0..minus.len()).step_by(holdable) {
        if i == 0 {
            max_dist = cmp::max(max_dist, minus[i]);
        }

        result += minus[i] * 2;
    }

    // 가장 긴 거리 한 번을 빼줌, (다놓고 돌아올 필요가 없음)
    result -= max_dist;

    write!(stdout, "{}", result).unwrap();
}
