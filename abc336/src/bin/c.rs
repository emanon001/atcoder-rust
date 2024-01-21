#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize
    };

    let mut v = Vec::new();
    let mut n = N - 1;
    loop {
        v.push(n % 5);
        n /= 5;
        if n == 0 {
            break;
        }
    }
    let ans = v.into_iter().rev().map(|x| x * 2).join("");
    println!("{}", ans);
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
