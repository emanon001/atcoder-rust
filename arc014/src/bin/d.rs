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
        all: i64, n: usize, m: usize,
        lv: [i64; n],
        xyv: [(i64, i64); m],
    };

    let head = lv[0] - 1;
    let tail = all - lv[n - 1];
    let mut length = lv
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| b - a - 1)
        .collect::<Vec<_>>();
    length.sort();
    let cusum = vec![0]
        .iter()
        .chain(length.iter())
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    for (x, y) in xyv {
        let l = x + y;
        let c = bsearch(-1, length.len() as i64, |i| {
            let i = i as usize;
            length[i] < l
        })
        .map(|i| i + 1)
        .unwrap_or(0) as usize;
        let mut res: i64 = n as i64;
        res += head.min(x);
        res += tail.min(y);
        res += cusum[c];
        res += (n - c - 1) as i64 * l;
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
