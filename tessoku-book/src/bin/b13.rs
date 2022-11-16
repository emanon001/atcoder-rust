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
        n: usize, k: i64,
        av: [i64; n]
    };

    let mut cusum = vec![0; n + 1];
    for i in 0..n {
        cusum[i + 1] += cusum[i] + av[i];
    }
    let mut res = 0;
    for i in 0..n {
        let j = bsearch(i as i64, cusum.len() as i64, |x| {
            let j = x as usize;
            cusum[j] - cusum[i] <= k
        })
        .unwrap_or(i as i64) as usize;
        res += j - i;
    }
    println!("{}", res);
}

fn solve2() {
    input! {
        n: usize, k: i64,
        av: [i64; n]
    };

    let mut cusum = vec![0; n + 1];
    for i in 0..n {
        cusum[i + 1] += cusum[i] + av[i];
    }
    let mut rv = vec![0; n];
    for i in 0..n {
        rv[i] = (if i == 0 { 0 } else { rv[i - 1] }).max(i);
        while rv[i] + 1 <= n && cusum[rv[i] + 1] - cusum[i] <= k {
            rv[i] += 1;
        }
    }
    let mut res = 0;
    for i in 0..n {
        res += rv[i] - i;
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
