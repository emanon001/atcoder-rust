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
        v: i64, a: i64, b: i64, c: i64
    };

    let mut rest = v % (a + b + c);
    rest -= a;
    if rest < 0 {
        println!("F");
        return;
    }
    rest -= b;
    if rest < 0 {
        println!("M");
        return;
    }
    println!("T");
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
