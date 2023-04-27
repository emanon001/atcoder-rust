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
        h: isize, w: isize,
        r: isize, c: isize,
    };

    let mut res = 0;
    if h > 1 && r > 1 {
        res += 1;
    }
    if h > 1 && r < h {
        res += 1;
    }
    if w > 1 && c > 1 {
        res += 1;
    }
    if w > 1 && c < w {
        res += 1;
    }
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
