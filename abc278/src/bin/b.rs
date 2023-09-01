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
        h: usize, m: usize,
    };

    for add in 0..60 * 24 {
        let minutes = (h * 60 + m + add) % (60 * 24);
        let new_h = minutes / 60;
        let new_m = minutes % 60;
        let a = new_h / 10;
        let b = new_h % 10;
        let c = new_m / 10;
        let d = new_m % 10;
        if a * 10 + c <= 23 && b * 10 + d <= 59 {
            println!("{}{} {}{}", a, b, c, d);
            return;
        }
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
