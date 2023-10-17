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
        n: usize, q: usize,
        events: [(usize, Usize1); q]
    };

    let mut cards = vec![(0, 0); n];
    for (c, x) in events {
        match c {
            1 => cards[x].0 += 1,
            2 => cards[x].1 += 1,
            3 => {
                let ans = if cards[x].0 >= 2 || cards[x].1 >= 1 {
                    "Yes"
                } else {
                    "No"
                };
                println!("{}", ans);
            },
            _ => unreachable!()
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
