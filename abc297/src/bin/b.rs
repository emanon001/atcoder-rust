#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use std::collections::*;
use std::io::{stdin, BufReader};

fn solve() {
    let mut source = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut source, $($tt)*)));
    input! {
        s: Chars
    };

    let mut b = vec![];
    for (i, ch) in s.iter().enumerate() {
        if ch == &'B' {
            b.push(i);
        }
    }
    let s = s
        .into_iter()
        .filter(|&ch| ch == 'K' || ch == 'R')
        .collect::<Vec<_>>();
    let ans = if b[0] % 2 != b[1] % 2 && (s[0] == 'R' && s[1] == 'K' && s[2] == 'R') {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
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
