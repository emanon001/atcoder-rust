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
        n: usize, q: usize,
        mut av: [i64; n],
        xv: [i64; q],
    };

    av.sort();
    let mut cusum = vec![0; n + 1];
    cusum[0] = 0;
    for i in 0..n {
        cusum[i + 1] = cusum[i] + av[i];
    }

    for x in xv {
        let i = bsearch(n as i64, -1 as i64, |i| {
            let i = i as usize;
            let a = av[i];
            a > x
        })
        .unwrap_or(0);
        // a <= x
        let mut res = 0_i64;
        let total = i * x;
        res += (total - cusum[i as usize]).abs();
        // a > x
        let total = (n as i64 - i) * x;
        res += (cusum[n] - cusum[i as usize] - total).abs();
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
