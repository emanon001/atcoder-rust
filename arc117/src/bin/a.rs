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
        a: isize, b: isize
    };

    let mut res = Vec::new();
    for i in 1..=a {
        res.push(i);
    }
    for i in 1..=b {
        res.push(-i);
    }
    let mut diff = 0_isize;
    for x in a.min(b) + 1..=a.max(b) {
        diff += x;
    }
    if a > b {
        res[(a + b - 1) as usize] -= diff;
    } else {
        res[(a - 1) as usize] += diff;
    }
    println!("{}", res.iter().join(" "));
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
