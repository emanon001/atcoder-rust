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
        abv: [(usize, usize); n]
    };

    let mut imos = vec![0_isize; 10.pow(6) + 10];
    for (a, b) in abv {
        imos[a] += 1;
        imos[b + 1] -= 1;
    }
    let mut res = 0_isize;
    let mut cur = 0_isize;
    for i in 1_isize..=10.pow(6) {
        cur += imos[i as usize];
        res += i * cur;
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
