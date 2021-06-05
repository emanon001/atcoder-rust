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
        a: i64, b: i64
    };

    let mut res = 1;
    for x in 2..b {
        let mut y = x;
        let mut count = 0;
        while y <= b {
            if y >= a {
                count += 1;
            }
            y += x;
        }
        if count >= 2 {
            res = x;
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
