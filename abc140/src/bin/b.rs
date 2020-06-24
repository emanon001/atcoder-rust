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
        av: [Usize1; n],
        bv: [usize; n],
        cv: [usize; n - 1],
    };

    let mut res = 0;
    let mut prev = n;
    for i in 0..n {
        let a = av[i];
        res += bv[a];
        if prev + 1 == a {
            res += cv[prev];
        }
        prev = a;
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
