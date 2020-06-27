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
        n: usize
    };

    let mut res = 0_u64;
    let mut v = vec![0_u64; n + 1];
    for a in 1..=n {
        let mut b = a;
        while b <= n {
            v[b] += 1;
            b += a;
        }
        res += a as u64 * v[a];
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
