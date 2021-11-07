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
        x: String
    };

    let v = x.split(".").collect::<Vec<_>>();
    let a = v[0];
    let b = v[1]
        .chars()
        .into_iter()
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();
    if b >= 5 {
        let res = a.parse::<usize>().unwrap() + 1;
        println!("{}", res);
    } else {
        println!("{}", a);
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
