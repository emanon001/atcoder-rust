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
        n: usize, k: isize, q: isize,
        av: [Usize1; q]
    };

    let mut solve_counts = vec![0_isize; n];
    for a in av {
        solve_counts[a] += 1;
    }
    for i in 0..n {
        let res = if k - q + solve_counts[i] > 0 {
            "Yes"
        } else {
            "No"
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
