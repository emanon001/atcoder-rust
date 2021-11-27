#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

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
        s: Chars,
        k: usize
    };

    let mut cusum = vec![0; s.len()];
    let mut cur = 0_usize;
    for i in 0..s.len() {
        if s[i] == '.' {
            cur += 1;
        }
        cusum[i] = cur;
    }

    let mut res = 0;
    for i in 0..s.len() {
        let j = bsearch(i as i64 - 1, s.len() as i64, |x| {
            let j = x as usize;
            let dot_count = cusum[j] - cusum[i] + if s[i] == '.' { 1 } else { 0 };
            dot_count <= k
        });
        let len = if let Some(j) = j {
            j - (i as i64) + 1
        } else {
            0
        };
        chmax!(res, len);
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
