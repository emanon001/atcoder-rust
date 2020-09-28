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
        s: Chars,
        t: Chars
    };

    let is_ok = s.into_iter().zip(t).all(|(x, y)| {
        let atcoder = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
        x == y || (x == '@' && atcoder.contains(&y)) || (y == '@' && atcoder.contains(&x))
    });
    let res = if is_ok {
        "You can win"
    } else {
        "You will lose"
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
