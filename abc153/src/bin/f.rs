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
        n: usize, d: u64, a: u64,
        mut xhv: [(u64, u64); n]
    };

    xhv.sort();
    let res = bsearch(10.pow(16), 0, |x| {
        let x = x as u64;
        let mut all_count = 0_u64;
        let mut bomb_area: VecDeque<(u64, u64)> = VecDeque::new();
        let mut cur_count = 0_u64;
        for &(x, h) in &xhv {
            while let Some((bx, c)) = bomb_area.pop_front() {
                if x > bx {
                    cur_count -= c;
                } else {
                    bomb_area.push_front((bx, c));
                    break;
                }
            }
            if cur_count * a >= h {
                continue;
            }
            let rest = h - cur_count * a;
            let c = (rest + a - 1) / a;
            all_count += c;
            cur_count += c;
            bomb_area.push_back((x + d * 2, c));
        }
        all_count <= x
    })
    .unwrap();
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
