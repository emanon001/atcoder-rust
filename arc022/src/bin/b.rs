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

    let mut last = vec![-1; 10.pow(5) + 1];
    let mut res = 1;
    let mut l = 0;
    for (i, a) in av.into_iter().enumerate() {
        if l as isize <= last[a] {
            l = last[a] as usize + 1;
        }
        last[a] = i as isize;
        let size = i - l + 1;
        if size > res {
            res = size;
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
