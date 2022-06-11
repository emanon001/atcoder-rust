#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn bsearch<F>(ok: f64, ng: f64, pred: F) -> Option<f64>
where
    F: Fn(f64) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while (ok - ng).abs() > 0.00001 {
        let mid = (ok + ng) / 2.0;
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
        av: [Usize1; k],
        xyv: [(f64, f64); n]
    };

    let res = bsearch(10.pow(6) as f64, 0.0, |a| {
        let mut ok_set = HashSet::new();
        for &i in &av {
            let (x, y) = xyv[i];
            for j in 0..n {
                let (x2, y2) = xyv[j];
                let d = ((x - x2).powf(2.0) + (y - y2).powf(2.0)).sqrt();
                if d <= a {
                    ok_set.insert(j);
                }
            }
        }
        ok_set.len() == n
    })
    .unwrap_or(0.0);
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
