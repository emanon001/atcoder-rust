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
        n: usize, k: u64,
        mut av: [u64; n]
    };

    let mut av = av.into_iter().enumerate().map(|(i, a)| (a, i)).collect::<Vec<_>>();
    av.sort();

    let mut res = vec![0_u64; n];
    for i in 0..n {
        let (_, pos) = av[i];
        let mut x = k / n as u64;
        let m = k % n as u64;
        x += if i < m as usize { 1 } else { 0 };
        res[pos] = x;
    }
    for x in res {
        println!("{}", x);
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
