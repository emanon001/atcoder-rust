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
        n: usize, m: usize,
        mut av: [i64; n],
        mut bv: [i64; m],
    };

    if m > n {
        println!("NO");
        return;
    }

    av.sort_by_key(|&a| -a);
    bv.sort_by_key(|&b| -b);
    let is_ok = av.into_iter().zip(bv).all(|(a, b)| a >= b);
    let res = if is_ok { "YES" } else { "NO" };
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
