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
        n: u128
    };

    for a in 1..=50 {
        for b in 1..=50 {
            let v = 3.pow(a) + 5.pow(b);
            if v > n {
                break;
            }
            if v == n {
                println!("{} {}", a, b);
                return;
            }
        }
    }
    println!("-1");
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
