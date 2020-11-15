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
        n: usize, w: i128,
        stpv: [(usize, usize, i128); n]
    };

    let mut imos = vec![0_i128; 10.pow(5) * 2 + 10];
    for (s, t, p) in stpv {
        imos[s] += p;
        imos[t] -= p;
    }
    let mut cur = 0_i128;
    for i in 0..=(10.pow(5) * 2) {
        cur += imos[i];
        if cur > w {
            println!("No");
            return;
        }
    }
    println!("Yes");
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
