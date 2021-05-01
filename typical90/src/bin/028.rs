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
        n: usize,
        rects: [(usize, usize, usize, usize); n]
    };

    let max_hw = 1000;
    let mut counts = vec![vec![0_isize; max_hw + 2]; max_hw + 2];
    for (lx, ly, rx, ry) in rects {
        counts[ly][lx] += 1;
        counts[ry][lx] -= 1;
        counts[ly][rx] -= 1;
        counts[ry][rx] += 1;
    }
    let mut cusum = vec![vec![0_isize; max_hw + 2]; max_hw + 2];
    for i in 0..=max_hw {
        for j in 0..=max_hw {
            cusum[i + 1][j + 1] = cusum[i + 1][j] + cusum[i][j + 1] + counts[i][j] - cusum[i][j];
        }
    }
    let mut res = vec![0; n + 1];
    for i in 0..=max_hw {
        for j in 0..=max_hw {
            let c = cusum[i + 1][j + 1];
            if c <= 0 {
                continue;
            }
            res[cusum[i + 1][j + 1] as usize] += 1;
        }
    }
    for k in 1..=n {
        println!("{}", res[k]);
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
