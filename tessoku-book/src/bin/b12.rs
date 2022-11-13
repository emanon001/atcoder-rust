#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn bsearch<F>(ok: f64, ng: f64, pred: F) -> Option<f64>
where
    F: Fn(f64) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while (ok - ng).abs() > 0.001 {
        let mid = (ok + ng) / 2.0;
        if pred(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    if ok == orig_ok {
        None
    } else {
        Some(ok)
    }
}

fn solve() {
    input! {
        n: f64,
    };

    let res = bsearch(0.0, n, |x| x.powf(3.0) + x <= n).unwrap();
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
