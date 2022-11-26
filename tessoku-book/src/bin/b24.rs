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
        mut xyv: [(i64, i64); n]
    };
    xyv.sort_by_key(|&(x, y)| (x, -y));

    let inf = 1 << 30;
    let mut dp = vec![inf; n];
    for &(_, y) in &xyv {
        let pos = bsearch(n as i64, -1, |i| {
            let i = i as usize;
            let y2 = dp[i];
            y <= y2
        })
        .unwrap_or(0) as usize;
        dp[pos] = y
    }
    let res = dp.into_iter().take_while(|v| v != &inf).count();
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
