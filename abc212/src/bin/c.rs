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

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize, m: usize,
        mut av: [i64; n],
        mut bv: [i64; m],
    };
    av.sort();
    bv.sort();

    let mut res = 1_64 << 60;
    for &a in &av {
        let b_i = bsearch(-1, m as i64, |x| {
            let b = bv[x as usize];
            b <= a
        });
        if let Some(b_i) = b_i {
            let b_i = b_i as usize;
            chmin!(res, (a - bv[b_i]).abs());
            let b_i = b_i + 1;
            if b_i < m {
                chmin!(res, (a - bv[b_i]).abs());
            }
        }
    }
    for b in bv {
        let a_i = bsearch(-1, n as i64, |x| {
            let a = av[x as usize];
            a <= b
        });
        if let Some(a_i) = a_i {
            let a_i = a_i as usize;
            chmin!(res, (b - av[a_i]).abs());
            let a_i = a_i + 1;
            if a_i < n {
                chmin!(res, (b - av[a_i]).abs());
            }
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
