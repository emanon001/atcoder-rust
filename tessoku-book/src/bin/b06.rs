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
        av: [usize; n],
        q: usize,
        lrv: [(usize, usize); q]
    };

    let mut cusum1 = vec![0; n + 1];
    let mut cusum2 = vec![0; n + 1];
    for i in 0..n {
        cusum1[i + 1] = cusum1[i] + if av[i] == 1 { 1 } else { 0 };
        cusum2[i + 1] = cusum2[i] + if av[i] == 1 { 0 } else { 1 };
    }

    for (l, r) in lrv {
        let count1 = cusum1[r] - cusum1[l - 1];
        let count2 = cusum2[r] - cusum2[l - 1];
        let res = if count1 > count2 {
            "win"
        } else if count1 < count2 {
            "lose"
        } else {
            "draw"
        };
        println!("{}", res);
    }
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
