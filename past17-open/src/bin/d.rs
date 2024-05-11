#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        A: u64, B: u64, C: u64,
        X: [u64; 12],
    };

    let mut free = 0_u64;
    let mut basic = B;
    let premium = C;
    for x in X {
        if x > 3 {
            free += (x - 3) * A;
        }
        if x > 50 {
            basic += (x - 50) * A;
        }
    }
    let ans = free.min(basic).min(premium);
    println!("{}", ans);
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
