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
        mut av: [i64; n]
    };

    let mut res = 0;
    for i in 0..n {
        let mut min = av[i];
        for j in i..n {
            if av[j] < min {
                min = av[j];
            }
            let score = (j - i + 1) as i64 * min;
            if score > res {
                res = score;
            }
        }
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
