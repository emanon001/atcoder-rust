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
        mut av: [i64; n],
        q: usize,
        bv: [i64; q]
    };

    av.sort();
    for b in bv {
        let l = bsearch(-1, n as i64, |x| {
            av[x as usize] <= b
        });
        let r = bsearch(n as i64, -1, |x| {
            av[x as usize] >= b
        });
        let res = match (l, r) {
            (Some(l), Some(r)) => {
                (av[l as usize] - b).abs().min(av[r as usize] - b).abs()
            }
            (Some(l), None) => {
                (av[l as usize] - b).abs()
            }
            (None, Some(r)) => {
                (av[r as usize] - b).abs()
            },
            _ => unreachable!()
        };
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
