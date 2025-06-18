// Baekjoon - 4179
// https://www.acmicpc.net/problem/4179

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
    let n: usize = io.get(0usize)?;
    let m: usize = io.get(0usize)?;

    let mut grid: Vec<Vec<char>> = vec![vec![' '; m]; n];
    let mut jihoon: (usize, usize) = (0, 0);

    for i in 0..n {
        let line: String = io.get_line()?;
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
            if c == 'J' {
                jihoon = (i, j);
            }
        }
    }

    let fire_grid = process_fire(&grid, n, m);

    let result: i64 = escape(&grid, &fire_grid, jihoon.0, jihoon.1, n, m) as i64;

    if result == -1 {
        io.put("IMPOSSIBLE").nl();
    } else {
        io.put(result).nl();
    }

    Some(())
}

fn escape(grid: &[Vec<char>], fire_grid: &[Vec<i32>], y: usize, x: usize, n: usize, m: usize) -> i32 {
    if y == 0 || x == 0 || y == n - 1 || x == m - 1 {
        return 1;
    }

    let movable: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited: Vec<Vec<i32>> = vec![vec![-1; m]; n];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    visited[y][x] = 1;
    queue.push_back((y, x));

    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        let cur_y = cur.0;
        let cur_x = cur.1;

        for (dy, dx) in movable {
            let ny = cur_y as i32 + dy;
            let nx = cur_x as i32 + dx;

            if ny < 0 || nx < 0 || ny >= n as i32 || nx >= m as i32 {
                return visited[cur_y][cur_x];
            }

            let ny = ny as usize;
            let nx = nx as usize;

            if visited[ny][nx] != -1 || grid[ny][nx] == '#' {
                continue;
            }

            let next_time = visited[cur_y][cur_x] + 1;

            if fire_grid[ny][nx] != -1 && fire_grid[ny][nx] <= next_time {
                continue;
            }

            visited[ny][nx] = next_time;
            queue.push_back((ny, nx));
        }
    }

    -1
}

fn process_fire(grid: &[Vec<char>], n: usize, m: usize) -> Vec<Vec<i32>> {
    let movable: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited: Vec<Vec<i32>> = vec![vec![-1; m]; n];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'F' {
                visited[i][j] = 1;
                queue.push_back((i, j));
            }
        }
    }

    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        let cur_y = cur.0;
        let cur_x = cur.1;

        for (dy, dx) in movable {
            let ny = cur_y as i32 + dy;
            let nx = cur_x as i32 + dx;

            if ny < 0 || nx < 0 || ny >= n as i32 || nx >= m as i32 {
                continue;
            }

            let ny = ny as usize;
            let nx = nx as usize;

            if visited[ny][nx] != -1 || grid[ny][nx] == '#' {
                continue;
            }

            visited[ny][nx] = visited[cur_y][cur_x] + 1;
            queue.push_back((ny, nx));
        }
    }

    visited
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
    print_disp!(usize, i64, String, &str, char);
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
            (self.r.read_line(&mut self.line).unwrap() > 0).then(|| {
                self.rem = unsafe { (&self.line[..] as *const str).as_ref().unwrap() };
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
