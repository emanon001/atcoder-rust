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
        mut sv: [i64; n],
        q: usize,
        kv: [i64; q]
    };

    sv.sort();

    for k in kv {
        let res = bsearch(10.pow(6) + 2, -1, |x| {
            let i = bsearch(sv.len() as i64, -1, |i| {
                let s = sv[i as usize];
                s > 0 && s >= x
            });
            let c = if let Some(i) = i { n as i64 - i } else { 0 };
            c <= k
        })
        .unwrap();
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
