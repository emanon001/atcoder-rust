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
        queries: [(usize, f64, f64); n]
    };

    let mut v = Vec::new();
    for &(q, l, r) in &queries {
        match q {
            1 => {
                v.push((l, r));
            }
            2 => {
                v.push((l, r - 0.1));
            }
            3 => {
                v.push((l + 0.1, r));
            }
            4 => {
                v.push((l + 0.1, r - 0.1));
            }
            _ => unreachable!()
        }
    }

    v.sort_by(|x1, x2| x1.partial_cmp(x2).unwrap());
    let mut res = 0;
    for i in 0..n {
        let (_, r) = v[i];
        let pos = bsearch(i as i64, n as i64, |x| {
            r >= v[x as usize].0
        });
        if let Some(pos) = pos {
            res += pos as usize - i;
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
