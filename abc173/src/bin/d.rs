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
        mut av: [u64; n]
    };

    av.sort_by_key(|&x| -(x as i64));
    let mut res = av[0];
    let mut rest = n - 2;
    let mut i = 1;
    while rest > 0 {
        rest -= 1;
        res += av[i];
        if rest > 0 {
            rest -= 1;
            res += av[i];
        }
        i += 1;
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
