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
    };

    let mut uncalled = (1..=n).collect::<BTreeSet<_>>();
    let mut called = BTreeSet::new();
    for _ in 0..q {
        input! {
            kind: usize,
        };
        match kind {
            1 => {
                let x = *uncalled.first().unwrap();
                uncalled.remove(&x);
                called.insert(x);
            },
            2 => {
                input! {
                    x: usize
                };
                called.remove(&x);
            },
            3 => {
                let x = *called.first().unwrap();
                println!("{}", x);
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
