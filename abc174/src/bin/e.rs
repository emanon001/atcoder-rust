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

// fn dfs(k: usize, av: &av, dp: &mut HashMap::new()) -> isize {
//     if dp[usize]
// }

fn solve() {
    input! {
        n: usize, k: usize,
        av: [f64; n]
    };

    let res = bsearch(10_i64.pow(9) + 1, 0 as i64, |x| {
        let max = x as f64;
        let mut count = 0_usize;
        for &a in &av {
            let n = (a / max) as usize;
            count += if a % max == 0.0 { n - 1 } else { n };
        }
        count <= k
    });
    let res = res.unwrap();
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
