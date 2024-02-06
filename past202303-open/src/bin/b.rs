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
        _: usize,
        A: String,
        B: String,
    };

    let av = A.split('.').collect_vec();
    let a1 = av[0].parse::<usize>().unwrap();
    let a2 = av[1].chars().map(|d| d.to_digit(10).unwrap()).collect_vec();

    let bv = B.split('.').collect_vec();
    let b1 = bv[0].parse::<usize>().unwrap();
    let b2 = bv[1].chars().map(|d| d.to_digit(10).unwrap()).collect_vec();

    let mut curry_up = false;
    let mut decimal_part = vec![];
    for (a, b) in a2.into_iter().zip(b2).rev() {
        let c = a + b + if curry_up { 1 } else { 0 };
        curry_up = c >= 10;
        decimal_part.push(c % 10);
    }
    decimal_part.reverse();

    let int_part = a1 + b1 + if curry_up { 1 } else { 0 };
    println!("{}.{}", int_part, decimal_part.iter().join(""));
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
