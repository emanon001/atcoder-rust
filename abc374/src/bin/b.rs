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
        S: Chars,
        T: Chars,
    };

    for (i, (s, t)) in S.iter().zip(T.iter()).enumerate() {
        if s != t {
            println!("{}", i + 1);
            return;
        }
    }

    if S.len() != T.len() {
        println!("{}", T.len().min(S.len()) + 1);
        return;
    }

    println!("0");
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
