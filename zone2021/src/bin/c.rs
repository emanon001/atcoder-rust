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
        abcdev: [[i64; 5]; n]
    };

    let res = bsearch(0, 10.pow(9) + 1, |x| {
        let mut cand = HashSet::new();
        for v in &abcdev {
            let mut bits = 0;
            for i in 0..5 {
                bits |= (if v[i] >= x { 1 } else { 0 }) << i;
            }
            cand.insert(bits);
        }
        let is_ok = |bits: i64| -> bool {
            (0..5).all(|i| (bits >> i) & 1 == 1)
        };
        let cand = cand.into_iter().collect::<Vec<_>>();
        let len = cand.len();
        for i in 0..len {
            if is_ok(cand[i]) {
                return true;
            }
            for j in i + 1..len {
                if is_ok(cand[i] | cand[j]) {
                    return true;
                }
                for k in j + 1..len {
                    if is_ok(cand[i] | cand[j] | cand[k]) {
                        return true;
                    }
                }
            }
        }
        false
    });

    println!("{}", res.unwrap());
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
