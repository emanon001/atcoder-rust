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

    let mut takahashi_wins = 0;
    let mut aoki_wins = 0;
    let mut ans = 'T';
    for ch in s {
        if ch == 'T' {
            takahashi_wins += 1;
            if takahashi_wins > aoki_wins {
                ans = 'T';
            }
        } else {
            aoki_wins += 1;
            if aoki_wins > takahashi_wins {
                ans = 'A';
            }
        }
    }
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
