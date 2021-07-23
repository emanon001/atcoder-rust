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
        s: Chars,
    };

    let mut ov = vec![0_i64; n];
    let mut xv = vec![0_i64; n];
    let mut oc = 0;
    let mut xc = 0;
    for i in 0..n {
        let ch = s[i];
        if ch == 'o' {
            oc += 1;
        } else {
            xc += 1;
        }
        ov[i] = oc;
        xv[i] = xc;
    }

    let mut res = 0_i64;
    for i in 0..n {
        let ch = s[i];
        let j = if ch == 'o' {
            let lc = xv[i];
            bsearch(-1_i64, n as i64, |x| {
                let r = x as usize;
                lc >= xv[r]
            })
        } else {
            // 'x'
            let lc = ov[i];
            bsearch(-1_i64, n as i64, |x| {
                let r = x as usize;
                lc >= ov[r]
            })
        };
        if let Some(j) = j {
            res += n as i64 - (j + 1_i64);
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
