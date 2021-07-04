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
        p: usize
    };

    let mut res = 0;
    let mut n = p;
    while n > 0 {
        let mut x = 1;
        let mut y = 1;
        while x * (y + 1) <= n {
            y += 1;
            x *= y;
        }
        res += 1;
        n -= x;
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
