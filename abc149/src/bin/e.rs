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
        n: usize, m: i64,
        mut av: [i64; n]
    };

    av.sort();
    let min_point = bsearch(-1, 10_i64.pow(5) * 2 + 10, |p| {
        let mut count = 0_i64;
        for i in 0..n {
            let x = av[i];
            let j = bsearch(n as i64, -1, |j| {
                let y = av[j as usize];
                let min_y = p - x;
                y >= min_y
            });
            if let Some(j) = j {
                count += n as i64 - j;
            }
        }
        count >= m
    })
    .unwrap();

    let cusum = av
        .iter()
        .rev()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<_>>();
    let mut res = 0_i64;
    let mut count = 0_i64;
    for i in 0..n {
        let x = av[i];
        let j = bsearch(n as i64, -1, |j| {
            let y = av[j as usize];
            let min_y = min_point - x;
            y >= min_y
        });
        if let Some(j) = j {
            let c = n as i64 - j;
            count += c;
            res += x * c + cusum[n - (j as usize + 1)];
        }
    }
    res -= (count - m) * min_point;
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
