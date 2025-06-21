// Baekjoon - 1504
// https://www.acmicpc.net/problem/1504

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
    let [n, e] = io.get([0usize, 0usize])?;
    let mut graph = vec![vec![]; n + 1];

    for _ in 0..e {
        let [from, to, weight] = io.get([0usize, 0usize, 0usize])?;
        graph[from].push((to, weight));
        graph[to].push((from, weight));
    }

    let [f1, f2] = io.get([0usize, 0usize])?;

    let dist_from_1 = dijkstra(&graph, 1);
    let dist_from_f1 = dijkstra(&graph, f1);
    let dist_from_f2 = dijkstra(&graph, f2);

    let path1 = match (dist_from_1[f1], dist_from_f1[f2], dist_from_f2[n]) {
        (Some(a), Some(b), Some(c)) => Some(a + b + c),
        _ => None,
    };

    let path2 = match (dist_from_1[f2], dist_from_f2[f1], dist_from_f1[n]) {
        (Some(a), Some(b), Some(c)) => Some(a + b + c),
        _ => None,
    };

    match (path1, path2) {
        (Some(p1), Some(p2)) => io.put(p1.min(p2)).nl(),
        (Some(p), None) | (None, Some(p)) => io.put(p).nl(),
        (None, None) => io.put(-1 as i64).nl(),
    };

    None
}

fn dijkstra(graph: &[Vec<(usize, usize)>], start: usize) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut dist = vec![None; n];
    let mut queue = BinaryHeap::new();

    dist[start] = Some(0);
    queue.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = queue.pop() {
        if dist[u].is_some() && dist[u].unwrap() < d {
            continue;
        }

        for &(v, w) in &graph[u] {
            let new_dist = d + w;
            if dist[v].is_none() || dist[v].unwrap() > new_dist {
                dist[v] = Some(new_dist);
                queue.push(Reverse((new_dist, v)));
            }
        }
    }

    dist
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
