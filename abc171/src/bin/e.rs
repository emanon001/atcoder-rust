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
        av: [usize; n]
    };

    let v = av[1..n].into_iter().fold(0, |acc, &x| acc ^ x);
    let mut res = vec![0; n];
    res[0] = v;
    let all = v ^ av[0];
    for i in 1..n {
        res[i] = av[i] ^ all;
    }
    println!("{}", res.into_iter().join(" "));
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
