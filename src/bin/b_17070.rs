// Baekjoon - 17070
// https://www.acmicpc.net/problem/17070

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
    let n = io.get(0usize)?;
    let grid = io.get(vec![vec![0usize; n]; n])?;

    // 0 - horizontal, 1 - vertical, 2 - diagonal
    // dp[i][j][파이프상태] = 경우의 수
    let mut dp = vec![vec![vec![0usize; 3]; n]; n];

    // 초기 상태: (0, 1)에 horizontal 파이프
    dp[0][1][0] = 1;

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 { continue; } // 벽인 경우 건너뛰기

            // 현재 위치에서 가능한 각 상태에 대해
            for state in 0..3 {
                if dp[i][j][state] == 0 { continue; }

                // 다음 가능한 이동들 계산
                let moves = get_possible_moves(&grid, i, j, n, state);

                for PipeState(next_state, ny, nx) in moves {
                    dp[ny][nx][next_state] += dp[i][j][state];
                }
            }
        }
    }

    // 목표 지점 (n-1, n-1)에 도달하는 모든 경우의 수 합산
    let result = dp[n - 1][n - 1][0] + dp[n - 1][n - 1][1] + dp[n - 1][n - 1][2];
    io.put(result).nl();

    None
}

struct PipeState(usize, usize, usize);  // (state, y, x)

fn get_possible_moves(
    grid: &[Vec<usize>],
    y: usize,
    x: usize,
    max_size: usize,
    current_state: usize,
) -> Vec<PipeState> {
    let mut moves = Vec::new();

    let right = x + 1 < max_size && grid[y][x + 1] == 0;
    let down = y + 1 < max_size && grid[y + 1][x] == 0;
    let diag = x + 1 < max_size && y + 1 < max_size &&
        grid[y][x + 1] == 0 && grid[y + 1][x] == 0 && grid[y + 1][x + 1] == 0;

    // 수평 이동 (현재가 수평 또는 대각선일 때)
    if right && (current_state == 0 || current_state == 2) {
        moves.push(PipeState(0, y, x + 1));
    }

    // 수직 이동 (현재가 수직 또는 대각선일 때)
    if down && (current_state == 1 || current_state == 2) {
        moves.push(PipeState(1, y + 1, x));
    }

    // 대각선 이동 (모든 상태에서 가능)
    if diag {
        moves.push(PipeState(2, y + 1, x + 1));
    }

    moves
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