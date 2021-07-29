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
    let mut counts = vec![vec![0_isize; max_hw + 1]; max_hw + 1];
    for (lx, ly, rx, ry) in rects {
        counts[lx][ly] += 1;
        counts[rx][ly] -= 1;
        counts[lx][ry] -= 1;
        counts[rx][ry] += 1;
    }
    for i in 0..max_hw {
        for j in 1..max_hw {
            counts[i][j] += counts[i][j - 1];
        }
    }
    for j in 0..max_hw {
        for i in 1..max_hw {
            counts[i][j] += counts[i - 1][j];
        }
    }
    let mut res = vec![0; n + 1];
    for i in 0..max_hw {
        for j in 0..max_hw {
            let c = counts[i][j];
            res[c as usize] += 1;
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
