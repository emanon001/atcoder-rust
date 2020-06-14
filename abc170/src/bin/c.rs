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
        x: isize, n: usize,
        pv: [isize; n]
    };

    let set = pv.into_iter().collect::<HashSet<_>>();
    let mut min = 101_isize;
    let mut res = 101_isize;
    for i in -1..=101 {
        if !set.contains(&i) {
            let x = (i - x).abs();
            if x < min {
                min = x;
                res = i;
            }
        }
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
