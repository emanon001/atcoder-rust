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
        n: usize
    };

    let mut v = Vec::new();
    for bits in 0..1 << 10 {
        let mut m = 0_i64;
        for i in 0..10 {
            m *= 10;
            m += if (bits >> i) & 1 == 1 { 4 } else { 7 };
        }
        v.push(m);
    }
    v.sort();
    println!("{}", v[n - 1]);
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
