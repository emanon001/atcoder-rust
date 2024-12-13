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
        B1: usize, R1: usize, B2: usize, R2: usize, T: usize,
    };

    let mut ans = 0;
    for t in 1..=T {
        let m1 = t % (B1 + R1);
        let b1 = (1..=B1).contains(&m1);
        let m2 = t % (B2 + R2);
        let b2 = (1..=B2).contains(&m2);
        if b1 && b2 {
            ans += 1;
        }
    }
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
