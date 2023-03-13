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
        n: usize, k: usize,
        pv: [i64; n]
    };

    let mut res = Vec::new();
    let mut set = BTreeSet::new();
    for i in 0..k {
        set.insert(pv[i]);
    }
    println!("{}", set.iter().next().unwrap());
    for i in k..n {
        let p = pv[i];
        let x = *set.iter().next().unwrap();
        if p > x {
            set.remove(&x);
            set.insert(p);
        }
        res.push(*set.iter().next().unwrap());
    }
    println!("{}", res.iter().join("\n"));
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
