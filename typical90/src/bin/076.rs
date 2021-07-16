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
        av: [i64; n]
    };

    let sum = av.iter().sum::<i64>();
    let av = av.repeat(2);
    let mut cusum = vec![0; n * 2 + 1];
    for i in 0..n * 2 {
        cusum[i + 1] = cusum[i] + av[i];
    }
    for i in 0..n {
        let l = cusum[i];
        let r = bsearch(i as i64, (n * 2 + 1) as i64, |x| {
            let x = x as usize;
            (cusum[x] - l) * 10 <= sum
        });
        if let Some(r) = r {
            let x = cusum[r as usize] - l;
            if x * 10 == sum {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
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
