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
        mut a: Chars,
        mut b: Chars,
    };

    a.reverse();
    b.reverse();

    for i in 0..(a.len().min(b.len())) {
        let x = a[i].to_digit(10).unwrap();
        let y = b[i].to_digit(10).unwrap();
        if x + y >= 10 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
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
