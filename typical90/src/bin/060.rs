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
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize,
        av: [i64; n]
    };

    let mut av1 = Vec::new();
    let mut prev = -1;
    for &a in &av {
        if a != prev {
            av1.push(a);
        }
        prev = a;
    }

    let av = av1;
    let n = av.len();

    let mut v1 = vec![1_i64 << 60; n + 1];
    v1[0] = 0;
    let mut c1 = vec![0; n + 1];
    for i in 0..n {
        let a = av[i];
        let j = bsearch(-1, n as i64, |x| {
            v1[x as usize] < a
        });
        if let Some(j) = j {
            let j = j as usize;
            v1[j + 1] = a;
            c1[i + 1] = std::cmp::max(c1[i], j + 1);
        } else {
            c1[i + 1] = c1[i];
        }
    }

    let mut v2 = vec![1_i64 << 60; n + 1];
    v2[0] = 0;
    let mut c2 = vec![0; n + 1];
    for i in 0..n {
        let a = av[n - i - 1];
        let j = bsearch(-1, n as i64, |x| {
            v2[x as usize] < a
        });
        if let Some(j) = j {
            let j = j as usize;
            v2[j + 1] = a;
            c2[i + 1] = std::cmp::max(c2[i], j + 1);
        } else {
            c2[i + 1] = c2[i];
        }
    }

    let mut res = 0;
    for i in 0..n {
        chmax!(res, c1[i] + c2[n - i]);
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
