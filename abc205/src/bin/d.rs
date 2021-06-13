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
        av: [i64; n],
        kv: [i64; q]
    };

    for k in kv {
        let res = bsearch(10_i64.pow(18) + n as i64 + 1, -1, |x| {
            // x以下の個数
            let i = bsearch(-1, n as i64, |i| {
                av[i as usize] <= x
            });
            let c = x - if i.is_none() { 0 } else { i.unwrap() + 1 };
            c >= k
        });
        println!("{}", res.unwrap());
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
