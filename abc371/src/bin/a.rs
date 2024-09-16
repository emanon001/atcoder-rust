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
        a: char, b: char, c: char,
    };

    let ans = match (a, b, c) {
        ('<', '<', '<') => "B",
        ('<', '<', '>') => "C",
        ('<', '>', '<') => unreachable!(),
        ('<', '>', '>') => "A",
        ('>', '<', '<') => "A",
        ('>', '<', '>') => unreachable!(),
        ('>', '>', '<') => "C",
        ('>', '>', '>') => "B",
        _ => unreachable!(),
    };

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
