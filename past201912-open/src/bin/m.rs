#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use ordered_float::OrderedFloat;
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
    while (ok - ng).abs() > 0.000001 {
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

fn main() {
    input! {
        n: usize, m: usize,
        abv: [(usize, usize); n],
        cdv: [(usize, usize); m],
    };

    let res = bsearch(0.0, 1000000000.0, |x| {
        let mut v = abv
            .iter()
            .map(|&(a, b)| (b as f64) - (a as f64 * x))
            .collect::<Vec<_>>();
        let mut v2 = cdv
            .iter()
            .map(|&(c, d)| (d as f64) - (c as f64 * x))
            .collect::<Vec<_>>();
        v2.sort_by_key(|&a| OrderedFloat::from(-a));
        v.push(v2[0]);
        v.sort_by_key(|&a| OrderedFloat::from(-a));
        let res = v.into_iter().take(5).sum::<f64>();
        res >= 0.0
    })
    .unwrap();
    println!("{}", res);
}
