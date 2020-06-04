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

fn add(a: u128, c: &mut [usize], digit: usize) {
    for i in 0..digit {
        if (a >> i) & 1 == 1 {
            c[i] += 1;
        }
    }
}

fn solve() {
    input! {
        n: usize,
        av: [u128; n]
    };

    let digit = 20;
    let empty = vec![0; digit];
    let cusum = av
        .into_iter()
        .scan(empty.clone(), |acc, x| {
            add(x, acc, digit);
            Some(acc.clone())
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for i in 0..n {
        let v = if i == 0 {
            &empty
        } else {
            &cusum[i - 1 as usize]
        };
        let j = bsearch(i as i64 - 1, n as i64, |x| {
            let x = x as usize;
            let v2 = &cusum[x as usize];
            (0..digit).all(|d| v2[d] - v[d] <= 1)
        })
        .unwrap();
        res += j as usize - i + 1;
    }
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
