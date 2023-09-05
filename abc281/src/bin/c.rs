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
        n: usize, t: i64,
        av: [i64; n]
    };

    let sum: i64 = av.iter().sum();
    let pos = t % sum;
    let mut cur = 0;
    for (i, a) in av.into_iter().enumerate() {
        if pos <= cur + a {
            println!("{} {}", i + 1, pos - cur);
            return;
        }
        cur += a;
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
