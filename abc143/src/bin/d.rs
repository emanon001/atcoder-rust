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
        mut lv: [usize; n]
    };

    lv.sort();
    let mut res = 0;
    for ai in 0..n - 2 {
        for bi in ai + 1..n - 1 {
            let maxc = lv[ai] + lv[bi] - 1;
            let ci = bsearch(bi as i64, lv.len() as i64, |j| {
                let j = j as usize;
                lv[j] <= maxc
            });
            match ci {
                Some(ci) => {
                    res += ci as usize - bi;
                }
                _ => {}
            }
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
