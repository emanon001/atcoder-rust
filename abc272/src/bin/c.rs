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
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        n: usize,
        av: [i64; n]
    };

    let odds = av
        .iter()
        .copied()
        .filter(i64::is_odd)
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    let evens = av
        .iter()
        .copied()
        .filter(i64::is_even)
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    let mut res = -1_i64;
    if odds.len() >= 2 {
        chmax!(res, odds[0] + odds[1]);
    }
    if evens.len() >= 2 {
        chmax!(res, evens[0] + evens[1]);
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
