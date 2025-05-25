// Baekjoon - 1103
// https://www.acmicpc.net/problem/1103

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
    let [r, c] = io.get([0usize; 2])?;
    let grid = io.get(vec![B; r])?;

    let mut memo = vec![vec![-1i32; c]; r];  // 메모이제이션 (-1: 미방문)
    let mut visiting = vec![vec![false; c]; r];  // 현재 DFS 경로에서 방문 중인지

    let result = dfs(&grid, 0, 0, r, c, &mut memo, &mut visiting);

    if result == -1 {
        io.put("-1").nl();
    } else {
        io.put(result as i64).nl();
    }

    None
}

fn dfs(
    grid: &[Vec<u8>],
    y: usize,
    x: usize,
    r: usize,
    c: usize,
    memo: &mut [Vec<i32>],
    visiting: &mut [Vec<bool>],
) -> i32 {
    if memo[y][x] != -1 {
        return memo[y][x];
    }

    // 현재 위치가 이미 방문 중이면 사이클 발생
    if visiting[y][x] {
        return -1;  // 무한히 움직일 수 있음
    }

    if grid[y][x] == b'H' {
        return 0;
    }

    visiting[y][x] = true;  // 현재 경로에서 방문 시작

    let movable = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let num = (grid[y][x] - b'0') as i32;
    let mut max_moves = 0;

    for (dy, dx) in movable {
        let ny = y as i32 + dy * num;
        let nx = x as i32 + dx * num;

        // 경계 체크
        if ny < 0 || ny >= r as i32 || nx < 0 || nx >= c as i32 {
            // 밖으로 나가면 게임 종료, 현재까지 이동 횟수 1 추가
            max_moves = max_moves.max(1);
            continue;
        }

        let ny = ny as usize;
        let nx = nx as usize;

        let result = dfs(grid, ny, nx, r, c, memo, visiting);

        if result == -1 {
            visiting[y][x] = false;
            return -1;
        }

        max_moves = max_moves.max(result + 1);
    }

    visiting[y][x] = false;  // 현재 경로에서 방문 종료
    memo[y][x] = max_moves;  // 메모이제이션

    max_moves
}

/// IO template - from bubbler (modified)
// boj - https://www.acmicpc.net/user/bubbler
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
    impl Fill for Vec<u8> {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            i.rem = i.rem.trim_start_matches(ws);
            while i.rem.is_empty() {
                i.next_line()?;
                i.rem = i.rem.trim_start_matches(ws);
            }
            let tok = i.rem.split(ws).next().unwrap();
            i.rem = &i.rem[tok.len()..];
            self.extend_from_slice(tok.as_bytes());
            Some(())
        }
    }
    pub(crate) const B: Vec<u8> = Vec::new();
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
pub fn main() {
    let stdin = stdin().lock();
    let stdout = stdout().lock();
    let mut io = IO::new(stdin, stdout);
    solve(&mut io);
}
