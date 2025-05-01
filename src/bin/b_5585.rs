// https://www.acmicpc.net/problem/5585
// Baekjoon - 5585

use std::io::{self, Read, Write};
fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let price: usize = lines.next().unwrap().parse().unwrap();
    let mut change = 1000 - price;

    let money = [500, 100, 50, 10, 5, 1];
    let mut result = 0;

    for coin in money {
        result += change / coin;
        change %= coin;
    }

    write!(stdout, "{}", result).unwrap();
}
