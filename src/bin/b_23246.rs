// Baekjoon - 23246
// https://www.acmicpc.net/problem/23246
use std::cmp::Ordering;
use std::io::{self, Read, Write};

#[derive(Debug)]
struct Climber {
    id: i32,
    lead: i32,
    spead: i32,
    boldering: i32,
}

impl Climber {
    fn total(&self) -> i32 {
        self.lead * self.spead * self.boldering
    }

    fn sum(&self) -> i32 {
        self.lead + self.spead + self.boldering
    }
}

impl Ord for Climber {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total()
            .cmp(&other.total())
            .then_with(|| self.sum().cmp(&other.sum()))
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Climber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Climber {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Climber {}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut climbers: Vec<Climber> = lines
        .take(n)
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            Climber {
                id: nums[0],
                lead: nums[1],
                spead: nums[2],
                boldering: nums[3],
            }
        })
        .collect();

    climbers.sort();

    let result = climbers
        .iter()
        .take(3)
        .map(|c| c.id.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    write!(io::stdout(), "{}", result)?;

    Ok(())
}
