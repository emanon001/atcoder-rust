#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn bsearch<F>(ok: i64, ng: i64, pred: F) -> Option<i64>
where
    F: Fn(i64) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
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
        n: i64, h: i64,
        a: i64, b: i64, c: i64, d: i64, e: i64
    };

    let res = bsearch(10_i64.pow(12), -1, |x| {
        (0..=n).any(|i| {
            let rest_x = x - a * i;
            if rest_x < 0 {
                return false;
            }
            let max = (rest_x / c).min(n - i);
            let res = bsearch(max + 1, -1, |j| {
                let satiation = h + b * i + d * j;
                (n - i - j) * e < satiation
            })
            .is_some();
            res
        })
    });
    println!("{}", res.unwrap());
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
