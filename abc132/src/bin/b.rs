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
        pv: [usize; n]
    };

    let res = pv
        .windows(3)
        .filter(|&v| {
            let p = v[1];
            let mut v = v.into_iter().collect::<Vec<_>>();
            v.sort();
            p == *v[1]
        })
        .count();
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
