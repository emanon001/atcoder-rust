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
        av: [usize; n]
    };

    let mut blocks = vec![1, 0, 0, 0];
    let mut res = 0;
    for a in av {
        for i in (0..blocks.len()).rev() {
            if i + a >= 4 {
                res += blocks[i];
            } else {
                blocks[i + a] = blocks[i];
            }
            blocks[i] = 0;
        }
        blocks[0] = 1;
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
