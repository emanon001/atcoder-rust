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
        xyv: [(f64, f64); n]
    };

    let mut res = 0_f64;
    for i in 0..n {
        let p1 = xyv[i];
        for j in i + 1..n {
            let p2 = xyv[j];
            let a = (p1.0 - p2.0).abs();
            let b = (p1.1 - p2.1).abs();
            let d = ((a * a) + (b * b)).sqrt();
            chmax!(res, d);
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
