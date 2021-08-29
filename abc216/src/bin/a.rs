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
        s: String
    };

    let mut v = s.split(".");
    let x: i32 = v.next().unwrap().to_string().parse().unwrap();
    let y: i32 = v.next().unwrap().to_string().parse().unwrap();
    let res = if 0 <= y && y <= 2 {
        format!("{}-", x)
    } else if 3 <= y && y <= 6 {
        format!("{}", x)
    } else {
        format!("{}+", x)
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
