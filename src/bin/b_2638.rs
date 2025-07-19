// Baekjoon - 2638
// https://www.acmicpc.net/problem/2638

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
    let [n, m] = io.get([0usize, 0usize])?;
    let mut graph = vec![vec![0usize; m]; n];
    let mut cheeses = Vec::new();

    for i in 0..n {
        for j in 0..m {
            let tmp = io.get(0usize)?;
            if tmp == 1 {
                cheeses.push((i, j));
                graph[i][j] = 1;
            }
        }
    }

    let mut time = 0;

    while !cheeses.is_empty() {
        mark_outside_air(&mut graph, n, m);

        let mut next_cheeses = Vec::new();

        for &(y, x) in &cheeses {
            if count_outside_air(&graph, y, x) < 2 {
                next_cheeses.push((y, x));
            } else {
                graph[y][x] = 0;
            }
        }

        cheeses = next_cheeses;
        reset_air_marks(&mut graph, n, m);
        time += 1;
    }

    io.put(time as i64);
    Some(())
}

fn mark_outside_air(graph: &mut [Vec<usize>], n: usize, m: usize) {
    let mut stack = Vec::new();

    for i in [0, n - 1] {
        for j in 0..m {
            if graph[i][j] == 0 {
                graph[i][j] = 2;
                stack.push((i, j));
            }
        }
    }
    for j in [0, m - 1] {
        for i in 1..n - 1 {
            if graph[i][j] == 0 {
                graph[i][j] = 2;
                stack.push((i, j));
            }
        }
    }

    while let Some((y, x)) = stack.pop() {
        for (dy, dx) in [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
            let ny = (y as i32 + dy) as usize;
            let nx = (x as i32 + dx) as usize;
            if ny < n && nx < m && graph[ny][nx] == 0 {
                graph[ny][nx] = 2;
                stack.push((ny, nx));
            }
        }
    }
}

fn count_outside_air(graph: &[Vec<usize>], y: usize, x: usize) -> usize {
    [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .filter_map(|&(dy, dx)| {
            let ny = (y as i32 + dy) as usize;
            let nx = (x as i32 + dx) as usize;
            graph.get(ny)?.get(nx)
        })
        .filter(|&&cell| cell == 2)
        .count()
}

fn reset_air_marks(graph: &mut [Vec<usize>], n: usize, m: usize) {
    for row in graph.iter_mut().take(n) {
        for cell in row.iter_mut().take(m) {
            if *cell == 2 {
                *cell = 0;
            }
        }
    }
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
