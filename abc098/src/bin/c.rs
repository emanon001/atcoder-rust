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
        s: Chars
    };

    let cusum = s
        .iter()
        .scan((0, 0), |acc, &ch| {
            if ch == 'W' {
                acc.0 += 1;
            } else {
                acc.1 += 1;
            };
            Some(*acc)
        })
        .collect::<Vec<_>>();

    let (_, max_r) = cusum[n - 1];
    let mut res = std::usize::MAX;
    for i in 0..n {
        let (l, r) = cusum[i];
        let c = if s[i] == 'W' { l - 1 } else { l } + max_r - r;
        if c < res {
            res = c;
        }
    }
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
