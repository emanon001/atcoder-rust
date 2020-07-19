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

fn main() {
    input! {
        n: usize, k: usize, c: usize,
        s: Chars
    };

    let mut forward_days = Vec::new();
    let mut last_ng_day = -1;
    for i in 0..n {
        if s[i] == 'x' {
            continue;
        }
        if i as isize <= last_ng_day {
            continue;
        }
        forward_days.push(i);
        last_ng_day = (i + c) as isize;
    }
    let mut backward_days = Vec::new();
    let mut last_ng_day = n as isize;
    for i in (0..n).rev() {
        if s[i] == 'x' {
            continue;
        }
        if i as isize >= last_ng_day {
            continue;
        }
        backward_days.push(i);
        last_ng_day = i as isize - c as isize;
    }

    for i in 0..n {
        let li = bsearch(-1, forward_days.len() as i64, |x| {
            let x = x as usize;
            i > forward_days[x]
        });
        let lc = if let Some(li) = li { li + 1 } else { 0 };
        let ri = bsearch(-1, backward_days.len() as i64, |x| {
            let x = x as usize;
            i < backward_days[x]
        });
        let rc = if let Some(ri) = ri { ri + 1 } else { 0 };
        let c = lc + rc
            - match (li, ri) {
                (Some(li), Some(ri)) => {
                    let l = forward_days[li as usize];
                    let r = backward_days[ri as usize];
                    if r - l > c {
                        0
                    } else {
                        -1
                    }
                }
                _ => 0,
            };
        if c < k as i64 {
            println!("{}", i + 1);
        }
    }
}
