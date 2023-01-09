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
        n: usize,
        mut cv: [i64; n],
        q: usize,
        xv: [i64; q]
    };

    cv.sort();
    let cusum = cv
        .into_iter()
        .scan(0_i64, |state, c| {
            *state += c;
            Some(*state)
        })
        .collect::<Vec<_>>();
    for x in xv {
        let res = bsearch(-1, n as i64, |i| {
            let i = i as usize;
            cusum[i] <= x
        })
        .map(|i| i + 1)
        .unwrap_or(0);
        println!("{}", res);
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
