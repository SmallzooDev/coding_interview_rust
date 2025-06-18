// Baekjoon - 2589
// https://www.acmicpc.net/problem/2589

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
    let n: usize = io.get(0usize)?;
    let m: usize = io.get(0usize)?;

    let mut grid: Vec<Vec<char>> = vec![vec![' '; m]; n];

    for i in 0..n {
        let line: String = io.get_line()?;
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    let mut max_treasure_distance = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'L' {
                let distance = get_max_distance(&grid, i, j, n, m);
                max_treasure_distance = max_treasure_distance.max(distance);
            }
        }
    }

    io.put(max_treasure_distance).nl();
    Some(())
}

fn get_max_distance(grid: &[Vec<char>], start_y: usize, start_x: usize, max_y: usize, max_x: usize) -> usize {
    let movable: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited: Vec<Vec<i32>> = vec![vec![-1; max_x]; max_y];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    visited[start_y][start_x] = 0;
    queue.push_back((start_y, start_x));

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let cur_y = current.0;
        let cur_x = current.1;

        for (dy, dx) in movable {
            let ny = cur_y as i32 + dy;
            let nx = cur_x as i32 + dx;

            if ny < 0 || nx < 0 || ny >= max_y as i32 || nx >= max_x as i32 {
                continue;
            }

            let ny = ny as usize;
            let nx = nx as usize;

            if visited[ny][nx] != -1 || grid[ny][nx] == 'W' {
                continue;
            }

            visited[ny][nx] = visited[cur_y][cur_x] + 1;
            queue.push_back((ny, nx));
        }
    }

    let mut max_distance: usize = 0;

    for i in 0..max_y {
        for j in 0..max_x {
            if visited[i][j] >= 0 && visited[i][j] as usize > max_distance {
                max_distance = visited[i][j] as usize;
            }
        }
    }

    max_distance
}

/// IO template - from bubbler (modified)
// boj - https://www.acmicpc.net/user/bubbler
#[allow(dead_code)]
mod io {
    pub(crate) use std::io::{stdin, stdout, BufRead, BufWriter, Write};
    pub(crate) struct IO<R: BufRead, W: Write> {
        ii: I<R>,
        oo: BufWriter<W>,
    }
    impl<R: BufRead, W: Write> IO<R, W> {
        pub(crate) fn new(r: R, w: W) -> Self {
            Self {
                ii: I::new(r),
                oo: BufWriter::new(w),
            }
        }
        pub(crate) fn get<T: Fill>(&mut self, exemplar: T) -> Option<T> {
            self.ii.get(exemplar)
        }
        pub(crate) fn get_line(&mut self) -> Option<String> {
            self.ii.get_line()
        }
        pub(crate) fn put<T: Print>(&mut self, t: T) -> &mut Self {
            t.print(&mut self.oo);
            self
        }
        pub(crate) fn nl(&mut self) -> &mut Self {
            self.put("\n")
        }
    }
    pub(crate) trait Print {
        fn print<W: Write>(&self, w: &mut W);
    }
    macro_rules! print_disp {
        ($($t:ty),+) => {
            $(impl Print for $t { fn print < W : Write > (& self, w : & mut W) {
            write!(w, "{}", self) .unwrap(); } })+
        };
    }
    print_disp!(usize, i64, String, & str, char);
    pub(crate) struct I<R: BufRead> {
        r: R,
        line: String,
        rem: &'static str,
    }
    impl<R: BufRead> I<R> {
        pub(crate) fn new(r: R) -> Self {
            Self {
                r,
                line: String::new(),
                rem: "",
            }
        }
        pub(crate) fn next_line(&mut self) -> Option<()> {
            self.line.clear();
            (self.r.read_line(&mut self.line).unwrap() > 0)
                .then(|| {
                    self
                        .rem = unsafe {
                        (&self.line[..] as *const str).as_ref().unwrap()
                    };
                })
        }
        pub(crate) fn get<T: Fill>(&mut self, exemplar: T) -> Option<T> {
            let mut exemplar = exemplar;
            exemplar.fill_from_input(self)?;
            Some(exemplar)
        }
        pub(crate) fn get_line(&mut self) -> Option<String> {
            self.next_line()?;
            Some(self.line.trim_end().to_string())
        }
    }
    pub(crate) trait Fill {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()>;
    }
    fn ws(c: char) -> bool {
        c <= ' '
    }
    macro_rules! fill_num {
        ($($t:ty),+) => {
            $(impl Fill for $t { fn fill_from_input < R : BufRead > (& mut self, i : &
            mut I < R >) -> Option < () > { i.rem = i.rem.trim_start_matches(ws); while i
            .rem.is_empty() { i.next_line() ?; i.rem = i.rem.trim_start_matches(ws); }
            let tok = i.rem.split(ws).next().unwrap(); i.rem = & i.rem[tok.len()..]; *
            self = tok.parse().ok() ?; Some(()) } })+
        };
    }
    fill_num!(usize, i64, f64);
    impl Fill for String {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            i.rem = i.rem.trim_start_matches(ws);
            while i.rem.is_empty() {
                i.next_line()?;
                i.rem = i.rem.trim_start_matches(ws);
            }
            let tok = i.rem.split(ws).next().unwrap();
            i.rem = &i.rem[tok.len()..];
            *self = tok.to_string();
            Some(())
        }
    }
    impl<T: Fill> Fill for Vec<T> {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            for ii in self.iter_mut() {
                ii.fill_from_input(i)?;
            }
            Some(())
        }
    }
    impl<T: Fill, const N: usize> Fill for [T; N] {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            for ii in self.iter_mut() {
                ii.fill_from_input(i)?;
            }
            Some(())
        }
    }
}

use io::*;
use std::collections::VecDeque;
pub fn main() {
    let stdin = stdin().lock();
    let stdout = stdout().lock();
    let mut io = IO::new(stdin, stdout);
    solve(&mut io);
}