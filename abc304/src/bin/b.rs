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
    };

    let res = if (0..10.pow(3)).contains(&n) {
        n
    } else if (10.pow(3)..10.pow(4)).contains(&n) {
        n - (n % 10)
    } else if (10.pow(4)..10.pow(5)).contains(&n) {
        n - (n % 100)
    } else if (10.pow(5)..10.pow(6)).contains(&n) {
        n - (n % 1000)
    } else if (10.pow(6)..10.pow(7)).contains(&n) {
        n - (n % 10000)
    } else if (10.pow(7)..10.pow(8)).contains(&n) {
        n - (n % 100000)
    } else if (10.pow(8)..10.pow(9)).contains(&n) {
        n - (n % 1000000)
    } else {
        unreachable!()
    };
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
