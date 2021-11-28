#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn mod_pow(n: u64, e: u64, _mod: u64) -> u64 {
    if e == 0 {
        return 1;
    }
    let n = n % _mod;
    let mut res = mod_pow(n, e >> 1, _mod);
    res = (res * res) % _mod;
    if e & 1 == 1 {
        res = (res * n) % _mod;
    }
    res
}

fn solve() {
    input! {
        n: u64, k: u64, m: u64,
    };

    let _mod = 998244353_u64;
    if m % _mod == 0 {
        println!("0");
        return;
    }

    let a = mod_pow(k, n, _mod - 1);
    let b = mod_pow(m, a, _mod);
    println!("{}", b);
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
