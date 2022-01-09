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
        n: usize, k: i64,
        av: [i64; n],
    };

    let mut cusum: HashMap<i64, Vec<usize>> = HashMap::new();
    let mut sum = 0_i64;
    for i in 0..n {
        sum += av[i];
        cusum.entry(sum).or_insert(Vec::new()).push(i);
    }
    let mut sum = 0_i64;
    let mut res = 0_i64;
    for i in 0..n {
        let diff = k + sum;
        let pos = cusum.get(&diff);
        if let Some(pos) = pos {
            let j = bsearch(pos.len() as i64, -1, |x| {
                let x = x as usize;
                pos[x] >= i
            });
            if let Some(j) = j {
                res += pos.len() as i64 - j;
            }
        }
        sum += av[i];
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
