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
        n: usize, k: i64,
        mut abv: [(i64, i64); n]
    };

    abv.sort();

    let mut i = 0;
    let mut k = k;
    for (a, b) in abv {
        if i + k >= a {
            k -= a - i;
            i = a;
            k += b;
        } else {
            println!("{}", i + k);
            return;
        }
    }
    println!("{}", i + k);
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
