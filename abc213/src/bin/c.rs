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
        _h: usize, _w: usize, n: usize,
        abv: [(usize, usize); n]
    };

    let mut av = HashSet::new();
    let mut bv = HashSet::new();
    for &(a, b) in &abv {
        av.insert(a);
        bv.insert(b);
    }
    let mut av = av.into_iter().collect::<Vec<_>>();
    let mut bv = bv.into_iter().collect::<Vec<_>>();
    av.sort();
    bv.sort();
    for (a, b) in abv {
        let a = bsearch(-1 as i64, av.len() as i64, |x| {
            av[x as usize] <= a
        }).unwrap_or(0) + 1;
        let b = bsearch(-1 as i64, bv.len() as i64, |x| {
            bv[x as usize] <= b
        }).unwrap_or(0) + 1;
        println!("{} {}", a, b);
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
