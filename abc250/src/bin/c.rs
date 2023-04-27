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
        n: usize, q: usize,
        xv: [Usize1; q]
    };

    let mut idx_to_value = (0..n).collect::<Vec<usize>>();
    let mut value_to_idx = (0..n).collect::<Vec<usize>>();
    for x in xv {
        let idx = value_to_idx[x];
        let idx2 = if idx == n - 1 { idx - 1 } else { idx + 1 };
        let y = idx_to_value[idx2];
        value_to_idx[x] = idx2;
        value_to_idx[y] = idx;
        idx_to_value[idx] = y;
        idx_to_value[idx2] = x;
    }
    println!("{}", idx_to_value.iter().map(|x| x + 1).join(" "));
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
