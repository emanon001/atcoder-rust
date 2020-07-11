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
        h: usize, w: usize, k: usize,
        grid: [Chars; h]
    };

    let mut cusum = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            let c = if grid[i][j] == '1' { 1 } else { 0 } + cusum[i + 1][j] + cusum[i][j + 1]
                - cusum[i][j];
            cusum[i + 1][j + 1] = c;
        }
    }

    let mut res = std::usize::MAX;
    for h_bits in 0..1 << (h - 1) {
        let mut range = Vec::new();
        let mut s = 0;
        for i in 0..(h - 1) {
            if (h_bits >> i) & 1 == 1 {
                range.push((s, i));
                s = i + 1;
            }
        }
        range.push((s, h - 1));
        let mut count = range.len() - 1;
        let mut l = 0;
        while l < w {
            let j = bsearch(l as i64 - 1, w as i64, |x| {
                let j = x as usize;
                range.iter().all(|&(s, e)| {
                    let c = cusum[e + 1][j + 1] + cusum[s][l] - cusum[e + 1][l] - cusum[s][j + 1];
                    c <= k
                })
            });
            if let Some(j) = j {
                l = j as usize + 1;
                if l < w {
                    count += 1;
                }
            } else {
                count = std::usize::MAX;
                break;
            }
        }
        if count < res {
            res = count;
        }
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
