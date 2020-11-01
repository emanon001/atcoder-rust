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
        n: usize, m: usize,
        mut hv: [i64; n],
        wv: [i64; m],
    };

    hv.sort();
    let mut f_cusum = vec![0; n / 2 + 1];
    let mut cur = 0_i64;
    for i in 0..(n / 2) {
        cur += (hv[i * 2] - hv[i * 2 + 1]).abs();
        f_cusum[i + 1] = cur;
    }
    let mut b_cusum = vec![0; n / 2 + 1];
    let mut cur = 0_i64;
    for i in 0..(n / 2) {
        cur += (hv[n - 1 - i * 2] - hv[n - 1 - i * 2 - 1]).abs();
        b_cusum[i + 1] = cur;
    }

    // println!("{:?}", f_cusum);
    // println!("{:?}", b_cusum);

    let mut res = std::i64::MAX;
    for w in wv {
        let i = bsearch(-1, n as i64, |x| {
            let x = x as usize;
            w >= hv[x]
        });
        let sum = if let Some(i) = i {
            let i = i as usize;
            if i % 2 == 0 {
                f_cusum[i / 2] + (hv[i] - w).abs() + b_cusum[n / 2 - (i / 2)]
            } else {
                f_cusum[(i + 1) / 2] + (w - hv[i + 1]).abs() + b_cusum[n / 2 - ((i + 1) / 2)]
            }
        } else {
            (w - hv[0]).abs() + b_cusum[n / 2]
        };
        if sum < res {
            res = sum;
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
