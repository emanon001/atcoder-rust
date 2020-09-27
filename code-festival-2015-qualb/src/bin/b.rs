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
        n: usize, m: usize,
        av: [usize; n]
    };

    let mut counts = vec![0; m + 1];
    for a in av {
        counts[a] += 1;
    }
    for a in 0..=m {
        let c = counts[a];
        if c * 2 > n {
            println!("{}", a);
            return;
        }
    }
    println!("?");
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
