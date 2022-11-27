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
        n: usize,
        tav: [(char, i64); n]
    };

    let m = 10000;
    let mut v = 0;
    for (t, a) in tav {
        match t {
            '+' => v += a,
            '-' => v -= a,
            '*' => v *= a,
            _ => unreachable!(),
        }
        v %= m;
        if v < 0 {
            v += m;
        }
        println!("{}", v);
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
