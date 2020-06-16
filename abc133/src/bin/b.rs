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
        n: usize, d: usize,
        xv: [[isize; d]; n]
    };

    let mut res = 0;
    for v in xv.into_iter().combinations(2) {
        let mut sum = 0_usize;
        for i in 0..d {
            sum += (v[0][i] - v[1][i]).pow(2) as usize;
        }
        let sqrt = num::integer::sqrt(sum);
        if sqrt * sqrt == sum {
            res += 1;
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
