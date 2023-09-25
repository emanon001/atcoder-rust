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
        n: usize, p: Usize1, q: Usize1, r: Usize1, _s: Usize1,
        mut av: [i64; n]
    };

    for i in 0..(q - p + 1) {
        av.swap(p + i, r + i);
    }
    println!("{}", av.iter().join(" "));
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
