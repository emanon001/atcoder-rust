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
        av: [i64; n]
    };

    let mut cusum = vec![0_i64; n];
    let mut c = 0;
    for i in 0..n {
        c += av[i];
        cusum[i] = c;
    }
    let mut cusum2 = vec![0_i64; n];
    let mut c = 0;
    for i in 0..n {
        c += cusum[i];
        cusum2[i] = c;
    }
    let mut max_v = vec![0_i64; n];
    let mut c = 0_i64;
    for i in 0..n {
        if av[i] > c {
            c = av[i];
        }
        max_v[i] = c;
    }
    for i in 0..n {
        let mut sum = cusum2[i];
        let max = max_v[i];
        let x = max;
        sum += x * (i + 1) as i64;
        println!("{}", sum)
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
