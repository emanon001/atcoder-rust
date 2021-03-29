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
        t: usize,
        cases: [u64; t]
    };

    for case in cases {
        let mut two_count = 0;
        let mut a = case;
        while a % 2 == 0 {
            two_count += 1;
            a /= 2;
        }
        let res = match two_count {
            0 => "Odd",
            1 => "Same",
            _ => "Even"
        };
        println!("{}", res);
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
