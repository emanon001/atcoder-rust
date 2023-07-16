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
        av: [Usize1; 3 * n]
    };

    let mut pos = vec![0; n];
    let mut count = vec![0; n];
    for (i, a) in av.into_iter().enumerate() {
        if count[a] == 1 {
            pos[a] = i;
        }
        count[a] += 1;
    }
    let res = (0..n).sorted_by_key(|a| pos[*a]).map(|a| a + 1).join(" ");
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
