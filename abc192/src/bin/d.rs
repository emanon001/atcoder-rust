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

fn f(x: &[i64], n: i64, m: i64) -> bool {
    let mut res = 0_i64;
    for i in 0..x.len() {
        if let Some(v) = res.checked_mul(n) {
            res = v;
        } else {
            return false;
        }
        res += x[i];
        if res > m {
            return false;
        }
    }
    res <= m
}

fn solve() {
    input! {
        x: Chars,
        m: i64
    };

    let x = x.into_iter().map(|ch| ch.to_digit(10).unwrap() as i64).collect::<Vec<_>>();
    if x.len() == 1 {
        let v = x[0];
        let res = if v > m {
            0
        } else {
            1
        };
        println!("{}", res);
        return;
    }
    let n = x.iter().max().unwrap() + 1;
    let res = bsearch(n - 1, 10_i64.pow(18) + 1, |y| {
        f(&x, y, m)
    });
    let res = if let Some(v) = res {
        v - n + 1
    } else {
        0
    };
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
