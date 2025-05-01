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

use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // let n: usize = lines.next().unwrap().parse().unwrap();

    let output = String::from("hello_world_!");
    write!(stdout, "{}", output).unwrap();
}
EOF
else
  echo "지원하지 않는 사이트입니다. (지원: leetcode, acmicpc.net)"
  exit 1
fi

echo "문제 생성 완료: src/bin/${file_name}.rs"
echo "실행 방법: cargo run --bin ${file_name}"
