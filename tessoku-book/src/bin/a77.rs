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
        n: usize, l: i64,
        k: usize,
        av: [i64; n]
    };

    let av = vec![0]
        .into_iter()
        .chain(av)
        .chain(vec![l])
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    let res = bsearch(0, 1_i64 << 60, |x| {
        let mut count = 0;
        let mut width = 0;
        for &a in &av {
            if count == k {
                width += a;
                continue;
            }
            if width >= x {
                count += 1;
                width = a;
            } else {
                width += a;
            }
        }
        count == k && width >= x
    })
    .unwrap_or(0);
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
