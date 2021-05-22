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
        av: [i64; n],
        bv: [i64; n],
        cv: [Usize1; n],
    };
    let mut table = vec![0; n + 1];
    for i in 0..n {
        table[bv[cv[i]] as usize] += 1;
    }
    let mut res = 0_i64;
    for i in 0..n {
        let a = av[i];
        res += table[a as usize];
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
