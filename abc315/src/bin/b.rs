#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        m: usize,
        d: [usize; m]
    };

    let sum: usize = d.iter().sum();
    let mid = sum / 2 + 1;
    let mut l = 0;
    for (i, d_i) in d.into_iter().enumerate() {
        let m = i + 1;
        let r = l + d_i;
        if (l..=r).contains(&mid) {
            println!("{} {}", m, mid - l);
            return;
        }
        l = r;
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
