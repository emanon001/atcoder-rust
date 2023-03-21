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

    let mut cur = 0;
    let mut v = vec![cur];
    for a in av {
        cur = (cur + a) % 360;
        v.push(cur);
    }
    v.sort();
    v.push(360);
    let mut res = 0;
    for (a, b) in v.into_iter().tuple_windows() {
        chmax!(res, b - a);
    }
    println!("{}", res)
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
