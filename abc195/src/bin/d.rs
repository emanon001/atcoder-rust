#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize, m: usize, q: usize,
        mut wvv: [(i64, i64); n],
        xv: [i64; m],
        queries: [(Usize1, Usize1); q]
    };

    wvv.sort_by_key(|&(_, v)| -v);
    for (l, r) in queries {
        let mut boxes = Vec::new();
        for i in 0..m {
            if l <= i && i <= r {
                continue;
            }
            boxes.push(i);
        }
        boxes.sort_by_key(|&i| xv[i]);
        let mut used_boxes = vec![false; m];
        let mut res = 0;
        for &(w, v) in &wvv {
            for &b in &boxes {
                if used_boxes[b] {
                    continue;
                }
                if xv[b] >= w {
                    res += v;
                    used_boxes[b] = true;
                    break;
                }
            }
        }
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
