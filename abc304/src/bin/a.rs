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
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        n: usize,
        sav: [(String, i64); n]
    };

    let mut min = 1_i64 << 60;
    let mut pos = 0;
    for i in 0..n {
        let (_, a) = sav[i].clone();
        if chmin!(min, a) {
            pos = i;
        }
    }
    for i in 0..n {
        println!("{}", sav[(pos + i) % n].0);
    }
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
