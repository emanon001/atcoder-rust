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
        av: [usize; n],
        q: usize,
        queries: [(Usize1, Usize1, usize); q]
    };

    let mut v = vec![Vec::new(); 2 * 10.pow(5) + 1];
    for i in 0..n {
        let a = av[i];
        v[a].push(i);
    }
    for (l, r, x) in queries {
        let xv = &v[x];
        let xvn = xv.len();
        let mut res = xvn;
        let li = bsearch(-1, xvn as i64, |i| {
            let i = i as usize;
            xv[i] < l
        });
        if let Some(li) = li {
            res -= li as usize + 1;
        }
        let ri = bsearch(xvn as i64, -1, |i| {
            let i = i as usize;
            xv[i] > r
        });
        if let Some(ri) = ri {
            res -= xvn - ri as usize;
        }
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
