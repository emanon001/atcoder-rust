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
        k: usize
    };

    let mut counts = vec![0; n + 1];
    for x in 2..=n {
        if counts[x] > 0 {
            continue;
        }
        let mut i = 1;
        while x * i <= n {
            counts[x * i] += 1;
            i += 1;
        }
    }
    let mut res = 0;
    for i in 0..=n {
        if counts[i] >= k {
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
