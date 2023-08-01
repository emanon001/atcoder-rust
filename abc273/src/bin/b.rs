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
        x: i64,
        k: usize
    };

    let mut x = x;
    for i in 0..k {
        let xs = x.to_string();
        if i >= xs.len() {
            break;
        }
        let n = xs[xs.len() - i - 1..].parse::<i64>().unwrap();
        if n >= 5 * 10.pow(i as u32) {
            x = x - n + 10.pow(i as u32 + 1);
        } else {
            x -= n;
        }
    }
    println!("{}", x);
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
