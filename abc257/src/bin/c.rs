#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

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
        wv: [i64; n]
    };

    let mut children = Vec::new();
    let mut adults = Vec::new();
    for i in 0..n {
        if s[i] == '0' {
            children.push(wv[i]);
        } else {
            adults.push(wv[i]);
        }
    }
    children.sort();
    adults.sort();
    let mut xv = Vec::new();
    for w in wv {
        xv.push(w);
        xv.push(w + 1);
    }

    let mut res = 0;
    for x in xv {
        let c1 = bsearch(-1, children.len() as i64, |i| {
            let i = i as usize;
            children[i] < x
        })
        .map(|i| i + 1)
        .unwrap_or(0);
        let c2 = bsearch(adults.len() as i64, -1, |i| {
            let i = i as usize;
            adults[i] >= x
        })
        .map(|i| adults.len() as i64 - i)
        .unwrap_or(0);
        let c = c1 + c2;
        chmax!(res, c);
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
