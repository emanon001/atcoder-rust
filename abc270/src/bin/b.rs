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
        x: isize, y: isize, z: isize
    };
    let max = 1000;
    let mut res = max;
    // x, _, _
    if 0 < x && x < y || y < x && x < 0 || x.is_positive() != y.is_positive() {
        chmin!(res, x.abs());
    }
    // z, y, x
    if 0 < z && z < y || y < z && z < 0 || z.is_positive() != y.is_positive() {
        chmin!(res, z.abs() + (z - x).abs());
    };
    if res == max {
        res = -1;
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
