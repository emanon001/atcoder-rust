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
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize, k: i64,
        abv: [(i64, i64); n]
    };

    let mut res = 0;
    for a_min in 1..100 {
        let a_max = a_min + k;
        for b_min in 1..100 {
            let b_max = b_min + k;
            let mut count = 0;
            for (a, b) in &abv {
                if (a_min..=a_max).contains(a) && (b_min..=b_max).contains(b) {
                    count += 1;
                }
            }
            chmax!(res, count);
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
