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
        av: [u64; n]
    };

    let cusum = av
        .into_iter()
        .scan(0_u64, |acc, a| {
            *acc += a;
            Some(*acc)
        })
        .collect::<Vec<_>>();
    let mut res = 0;
    for i in 0..n {
        let ia = if i == 0 { 0 } else { cusum[i - 1] };
        let j = bsearch(n as i64, i as i64 - 1, |x| {
            let x = x as usize;
            cusum[x] - ia >= k
        });
        if let Some(j) = j {
            res += n - j as usize;
        } else {
            break;
        }
    }
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
