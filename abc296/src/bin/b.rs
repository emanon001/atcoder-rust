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
        s: [Chars; 8]
    };

    let cols = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let rows = (1..=8).rev().collect::<Vec<_>>();
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                println!("{}{}", cols[j], rows[i]);
                return;
            }
        }
    }
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
