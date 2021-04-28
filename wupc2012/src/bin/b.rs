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
        sv: [String; n]
    };

    let mut cand = Vec::new();
    for i in 0..n - 1 {
        for j in i + 1..n {
            cand.push(format!("{}{}", sv[i], sv[j]));
            cand.push(format!("{}{}", sv[j], sv[i]));
        }
    }
    cand.sort();
    println!("{}", cand[0]);
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
