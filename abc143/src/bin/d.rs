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
        mut lv: [usize; n]
    };

    let abv = lv
        .iter()
        .combinations(2)
        .map(|v| (*v[0], *v[1]))
        .collect::<Vec<_>>();
    lv.sort();
    let mut res = 0;
    for (a, b) in abv {
        let cmin = a.max(b) - a.min(b) + 1;
        let cmax = a + b - 1;
        let minj = bsearch(lv.len() as i64, -1, |j| {
            let j = j as usize;
            cmin <= lv[j]
        });
        let maxj = bsearch(-1, lv.len() as i64, |j| {
            let j = j as usize;
            lv[j] <= cmax
        });
        match (minj, maxj) {
            (Some(minj), Some(maxj)) => {
                let mut c = maxj as usize - minj as usize + 1;
                if a >= cmin {
                    c -= 1;
                }
                if b >= cmin {
                    c -= 1;
                }
                res += c;
            }
            _ => {}
        }
    }
    println!("{}", res / 3);
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
