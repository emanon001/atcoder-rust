#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        q: usize,
    };

    let mut set = BTreeSet::new();
    for _ in 0..q {
        input! {
            kind: usize, x: i64
        };
        match kind {
            1 => {
                set.insert(x);
            }
            2 => {
                if set.is_empty() {
                    println!("-1");
                    continue;
                }
                let max = 1_i64 << 60;
                let a = (set.range(..x).next_back().unwrap_or(&max) - x).abs();
                let b = (set.range(x..).next().unwrap_or(&max) - x).abs();
                let res = a.min(b);
                println!("{}", res);
            }
            _ => unreachable!(),
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
