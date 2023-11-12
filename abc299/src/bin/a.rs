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
        _: usize,
        s: Chars
    };

    let l = s.iter().position(|&ch| ch == '|').unwrap();
    let r = s.iter().rposition(|&ch| ch == '|').unwrap();
    let i = s.iter().rposition(|&ch| ch == '*').unwrap();
    let ans = if ((l + 1)..r).contains(&i) {
        "in"
    } else {
        "out"
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
