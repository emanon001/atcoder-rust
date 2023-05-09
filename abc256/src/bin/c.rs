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
        h1: usize, h2: usize, h3: usize, w1: usize, w2: usize, w3: usize
    };

    let mut res = 0;
    for comb in (1..h1).combinations(2) {
        let mut comb = comb;
        comb.sort();
        let a = comb[0];
        let b = comb[1] - comb[0];
        let c = h1 - comb[1];
        for comb2 in (1..h2).combinations(2) {
            let mut comb2 = comb2;
            comb2.sort();
            let a2 = comb2[0];
            let b2 = comb2[1] - comb2[0];
            let c2 = h2 - comb2[1];
            if a + a2 >= w1 {
                continue;
            }
            let a3 = w1 - a - a2;
            if b + b2 >= w2 {
                continue;
            }
            let b3 = w2 - b - b2;
            if c + c2 >= w3 {
                continue;
            }
            let c3 = w3 - c - c2;
            if a3 + b3 + c3 == h3 {
                res += 1;
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
