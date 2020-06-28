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
        n: usize, k: u64,
        mut av: [u64; n],
        mut fv: [u64; n],
    };

    av.sort();
    fv.sort_by_key(|&x| -(x as i64));
    let res = bsearch(10_i64.pow(18), -1, |x| {
        let mut rest = k as i64;
        for i in 0..n {
            let a = av[i];
            let max_a = x as u64 / fv[i];
            if a > max_a {
                rest -= (a - max_a) as i64;
            }
        }
        rest >= 0
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
