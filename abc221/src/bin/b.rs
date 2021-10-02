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
        mut s: Chars,
        t: Chars,
    };

    if s == t {
        println!("Yes");
        return;
    }

    for i in 0..s.len() - 1 {
        s.swap(i, i + 1);
        if s == t {
            println!("Yes");
            return;
        }
        s.swap(i, i + 1);
    }
    println!("No");
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
