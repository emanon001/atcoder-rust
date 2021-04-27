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
        n: i64
    };

    let d = n.to_string().len();
    let mut res = 0;
    for x in 1..10.pow(d as u32 / 2) {
        let s = x.to_string();
        let x: i64 = format!("{}{}", s, s).parse::<i64>().unwrap();
        if x <= n {
            res += 1;
        }
    }
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
