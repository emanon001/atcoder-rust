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
        a: isize, b: isize, c: isize, d: isize, e: isize, f: isize, x: isize
    };

    let c1 = x / (a + c) * a;
    let d1 = b * (c1 + (x % (a + c)).min(a));
    let c2 = x / (d + f) * d;
    let d2 = e * (c2 + (x % (d + f)).min(d));
    let res = if d1 > d2 {
        "Takahashi"
    } else if d1 < d2 {
        "Aoki"
    } else {
        "Draw"
    };
    println!("{}", res);
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
