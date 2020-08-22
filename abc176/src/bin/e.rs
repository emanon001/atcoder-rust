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
        h: usize, w: usize, m: usize,
        hwv: [(Usize1, Usize1); m]
    };

    let mut sum_h = vec![0_i64; h];
    let mut sum_w = vec![0_i64; w];
    for &(i, j) in &hwv {
        sum_h[i] += 1;
        sum_w[j] += 1;
    }

    let mut max = 0;
    for i in 0..h {
        if sum_h[i] >= max {
            max = sum_h[i];
        }
    }
    let mut max_h_set = HashSet::new();
    for i in 0..h {
        if sum_h[i] == max {
            max_h_set.insert(i);
        }
    }

    let mut max = 0;
    for i in 0..w {
        if sum_w[i] >= max {
            max = sum_w[i];
        }
    }
    let mut max_w_set = HashSet::new();
    for i in 0..w {
        if sum_w[i] == max {
            max_w_set.insert(i);
        }
    }

    let bomb_set = hwv.into_iter().collect::<HashSet<_>>();
    let mut res = 0;
    for &h in &max_h_set {
        for &w in &max_w_set {
            if !bomb_set.contains(&(h, w)) {
                res = sum_h[h] + sum_w[w];
                break;
            } else {
                res = sum_h[h] + sum_w[w] - 1;
            }
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
