// Baekjoon - 2204
// https://www.acmicpc.net/problem/2204

use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let n: usize = line.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut words = Vec::new();

        for _ in 0..n {
            let mut word = String::new();
            stdin.read_line(&mut word).unwrap();
            words.push(word.trim().to_string());
        }

        let mut min_word = &words[0];

        for word in &words[1..] {
            if word.to_lowercase() < min_word.to_lowercase() {
                min_word = word;
            }
        }

        writeln!(stdout, "{}", min_word).unwrap();
    }
}
