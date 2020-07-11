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
        n: usize, k: usize,
        mut av: [i64; n]
    };

    av.sort();
    let res = bsearch(10.pow(18) + 1, -10.pow(18) - 1, |x| {
        let mut count = 0;
        for i in 0..n {
            let a = av[i];
            if a >= 0 {
                let j = bsearch(i as i64, n as i64, |j| {
                    let j = j as usize;
                    let b = av[j];
                    a * b <= x
                });
                if let Some(j) = j {
                    count += j as usize - i;
                }
            } else {
                let j = bsearch(n as i64, i as i64, |j| {
                    let j = j as usize;
                    let b = av[j];
                    a * b <= x
                });
                if let Some(j) = j {
                    count += n - j as usize;
                }
            }
        }
        count >= k
    });
    println!("{}", res.unwrap());
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
