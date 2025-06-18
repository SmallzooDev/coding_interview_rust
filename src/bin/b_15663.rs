// Baekjoon - 15663
// https://www.acmicpc.net/problem/15663

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
    let [n, m] = io.get([0usize, 0usize])?;
    let mut sequence = Vec::new();
    let mut nums = io.get(vec![0usize; n])?;
    let mut visited = vec![false; n];
    nums.sort();

    comb(&nums, m, &mut sequence, &mut visited, io);
    None
}

fn comb<R: BufRead, W: Write>(
    nums: &[usize],
    m: usize,
    sequence: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    io: &mut IO<R, W>,
) {
    if sequence.len() == m {
        for (i, &num) in sequence.iter().enumerate() {
            if i > 0 {
                io.put(" ");
            }
            io.put(num);
        }
        io.nl();
        return;
    }

    for i in 0..nums.len() {
        if !visited[i] && (i == 0 || nums[i] != nums[i - 1] || visited[i - 1]) {
            visited[i] = true;
            sequence.push(nums[i]);
            comb(nums, m, sequence, visited, io);
            sequence.pop();
            visited[i] = false;
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
