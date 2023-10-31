#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{stdin, BufReader};

fn solve() {
    let mut source = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut source, $($tt)*)));
    input! {
        n: usize, m: usize,
        s: [Chars; n]
    };

    for i in 0..n {
        for j in 0..m {
            if check(i, j, &s) {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}

fn check(i: usize, j: usize, s: &Vec<Vec<char>>) -> bool {
    if i + 9 > s.len() || j + 9 > s[0].len() {
        return false;
    }
    // 左上の3x3マスが全て黒か
    for i in i..i + 3 {
        for j in j..j + 3 {
            if s[i][j] == '.' {
                return false;
            }
        }
    }
    // 右下の3x3マスが全て黒か
    for i in i + 6..i + 9 {
        for j in j + 6..j + 9 {
            if s[i][j] == '.' {
                return false;
            }
        }
    }
    // 左上の3x3マスに八方位で隣接するマスが全て白か
    for i in i..i + 3 {
        if s[i][j + 3] == '#' {
            return false;
        }
    }
    for j in j..j + 4 {
        if s[i + 3][j] == '#' {
            return false;
        }
    }
    // 右下の3x3マスに八方位で隣接するマスが全て白か
    for j in j + 5..j + 9 {
        if s[i + 5][j] == '#' {
            return false;
        }
    }
    for i in i + 6..i + 9 {
        if s[i][j + 5] == '#' {
            return false;
        }
    }
    true
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
