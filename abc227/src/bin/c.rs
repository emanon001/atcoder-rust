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
        n: u128
    };

    let mut res = 0;
    for a in 1..10.pow(6) {
        for b in a..10.pow(6) {
            let ab = a * b;
            if ab * b > n {
                break;
            }
            let c = n / ab;
            if c >= b {
                res += c - b + 1;
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
