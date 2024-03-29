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

#[allow(non_snake_case)]
fn solve() {
    input! {
        N: usize, M: usize, K: usize, Q: i64,
        mut PT: [(i64, usize); N],
    };

    PT.sort();
    let mut cusum1 = vec![0];
    let mut cusum2 = vec![0];
    for (p, t) in PT {
        if t == 0 {
            cusum1.push(cusum1.last().unwrap() + p);
        } else {
            cusum2
                .push(cusum2.last().unwrap() + p + if (cusum2.len() - 1) % K == 0 { Q } else { 0 });
        }
    }

    let ans = bsearch(10_i64.pow(18), 0, |x| {
        for i in 0..cusum1.len() {
            if i > M {
                break;
            }
            if cusum2.len() <= M - i {
                continue;
            }
            if cusum1[i] + cusum2[M - i] <= x {
                return true;
            }
        }
        false
    })
    .unwrap();
    println!("{}", ans);
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
