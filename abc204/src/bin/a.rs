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
        x: usize, y: usize
    };

    let res = if x == y {
        x
    } else {
        let mut v = HashSet::new();
        v.insert(0);
        v.insert(1);
        v.insert(2);
        v.remove(&x);
        v.remove(&y);
        v.into_iter().next().unwrap()
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
