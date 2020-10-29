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
        n: usize, a: i64, b: i64,
        sdv: [(String, i64); n]
    };

    let mut res = 0_i64;
    for (s, d) in sdv {
        let d = if d < a {
            a
        } else if a <= d && d <= b {
            d
        } else {
            b
        };
        res += if s == "East" { d } else { -d };
    }
    if res == 0 {
        println!("0");
    } else if res > 0 {
        println!("East {}", res.abs());
    } else {
        println!("West {}", res.abs());
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
