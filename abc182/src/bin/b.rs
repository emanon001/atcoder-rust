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
        av: [u64; n]
    };

    let mut max_c = 0;
    let mut res = 0;
    for k in 2..=1000 {
        let mut c = 0;
        for &a in &av {
            if a % k == 0 {
                c += 1;
            }
        }
        if c >= max_c {
            res = k;
            max_c = c;
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
