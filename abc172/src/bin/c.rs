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
        n: usize, m: usize, k: usize,
        av:[usize; n],
        bv:[usize; m],
    };

    let acusum = av
        .into_iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<_>>();
    let bcusum = bv
        .into_iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<_>>();
    let mut res = 0;
    for i in 0..=n {
        let asum = if i == 0 { 0 } else { acusum[i - 1] };
        if asum > k {
            continue;
        }
        let rest = k - asum;
        let j = bsearch(-1, m as i64, |x| {
            let x = x as usize;
            bcusum[x] <= rest
        });
        let sum = i + j.map(|j| j as usize + 1).unwrap_or(0);
        if sum > res {
            res = sum;
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
