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
        av: [usize; n]
    };

    let sorted = av.iter().sorted().copied().collect::<Vec<_>>();
    let mut actual = vec![0; n];
    for i in 0..k {
        let mut v = Vec::new();
        let mut pos = i;
        while pos < n {
            v.push(av[pos]);
            pos += k;
        }
        v.sort();
        let mut pos = i;
        for a in v {
            actual[pos] = a;
            pos += k;
        }
    }
    let res = if actual == sorted { "Yes" } else { "No" };
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
