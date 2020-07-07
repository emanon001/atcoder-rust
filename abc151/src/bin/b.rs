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
        n: usize, k: usize, m: usize,
        av: [usize; n - 1]
    };

    let required = n * m;
    let sum = av.iter().sum::<usize>();
    let res = if sum + k < required {
        -1
    } else if sum >= required {
        0 as isize
    } else {
        (required - sum) as isize
    };
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
