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
        hv: [isize; n]
    };

    let mut res = 0;
    let mut max = 0;
    for (i, h) in hv.into_iter().enumerate() {
        if chmax!(max, h) {
            res = i + 1;
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
