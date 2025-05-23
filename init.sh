#!/bin/bash

url=$1

if [ -z "$url" ]; then
  echo "사용법: ./init.sh <문제_URL>"
  exit 1
fi

# 문자열을 snake_case로 변환하고 패키지 이름으로 적합하게 만드는 함수
to_snake_case() {
  # 숫자로 시작하는 경우 'p' 접두사 추가
  local input=$1
  if [[ $input =~ ^[0-9] ]]; then
    input="p$input"
  fi

  # 특수문자를 제거하고 snake_case로 변환
  echo "$input" | sed 's/[^a-zA-Z0-9-]//g' |
    sed 's/[A-Z]/\L&/g' |
    sed 's/-/_/g'
}

# URL에서 사이트 타입 확인
if [[ $url == *"leetcode.com"* ]]; then
  # URL에서 problems/ 이후의 문제 이름만 추출
  raw_name=$(echo $url | sed -E 's/.*problems\/([^/?]+).*/\1/')
  problem_name=$(to_snake_case "$raw_name")

  # 기본 디렉토리 경로
  file_name="l_${problem_name}"

  # 디렉토리 생성
  mkdir -p "src/bin"

  # Rust 실행 파일 생성
  cat >"src/bin/${file_name}.rs" <<EOF
// LeetCode - ${raw_name}
// ${url}

fn main() {
    println!("문제: ${raw_name}");
    
    // 여기에 코드 작성
    let solution = Solution::new();
    
    println!("완료!");
}

struct Solution;

impl Solution {
    fn new() -> Self {
        Solution
    }
    
    // 문제 풀이 메서드 추가
}
EOF
elif [[ $url == *"acmicpc.net"* ]]; then
  raw_name=$(echo $url | sed -E 's/.*\/problem\/([0-9]+).*/\1/')

  # 기본 디렉토리 경로
  file_name="b_${raw_name}"

  # 디렉토리 생성
  mkdir -p "src/bin"

  # Rust 실행 파일 생성 (백준용 템플릿)
  cat >"src/bin/${file_name}.rs" <<EOF
// Baekjoon - ${raw_name}
// ${url}

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
    // let n: usize = io.get(0usize)?;
    // let s: String = io.get(String::new())?;
    // let line: String = io.get_line()?;
    
    // 여기에 문제 풀이 코드 작성
    
    // io.put("결과").nl();
    None
}

/// IO template - from bubbler (modified)
// boj - https://www.acmicpc.net/user/bubbler
mod io {
    pub(crate) use std::io::{Write, stdin, stdout, BufWriter, BufRead};
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
        (\$(\$t:ty),+) => {
            \$(impl Print for \$t { fn print < W : Write > (& self, w : & mut W) {
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
        (\$(\$t:ty),+) => {
            \$(impl Fill for \$t { fn fill_from_input < R : BufRead > (& mut self, i : &
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
pub fn main() {
    let stdin = stdin().lock();
    let stdout = stdout().lock();
    let mut io = IO::new(stdin, stdout);
    solve(&mut io);
}
EOF
else
  echo "지원하지 않는 사이트입니다. (지원: leetcode, acmicpc.net)"
  exit 1
fi

echo "문제 생성 완료: src/bin/${file_name}.rs"
echo "실행 방법: cargo run --bin ${file_name}"
