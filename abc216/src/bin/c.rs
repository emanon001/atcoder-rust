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
        n: usize
    };

    let mut res = VecDeque::new();
    let mut n = n;
    while n > 1 {
        if n % 2 == 0 {
            res.push_front('B');
            n /= 2;
        } else {
            res.push_front('A');
            n -= 1;
        }
    }
    res.push_front('A');
    println!("{}", res.iter().join(""));
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
