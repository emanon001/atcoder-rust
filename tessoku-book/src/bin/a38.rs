#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        d: usize, n: usize,
        lrhv: [(Usize1, Usize1, usize); n]
    };

    let mut dp = vec![24; d];
    for (l, r, h) in lrhv {
        for i in l..=r {
            chmin!(dp[i], h);
        }
    }
    let res = dp.into_iter().sum::<usize>();
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
