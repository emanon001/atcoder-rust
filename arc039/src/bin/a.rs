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
        a: isize, b: isize
    };

    let av = a
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as isize)
        .collect::<Vec<_>>();
    let bv = b
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as isize)
        .collect::<Vec<_>>();
    let mut res = -1_isize << 30;
    for i in 0..3 {
        let a = av
            .iter()
            .enumerate()
            .fold(0, |acc, (ai, &x)| acc * 10 + if ai == i { 9 } else { x });
        let diff = a - b;
        if diff > res {
            res = diff;
        }
    }
    for i in 0..3 {
        let b = bv.iter().enumerate().fold(0, |acc, (bi, &x)| {
            acc * 10
                + if bi == i {
                    if bi == 0 {
                        1
                    } else {
                        0
                    }
                } else {
                    x
                }
        });
        let diff = a - b;
        if diff > res {
            res = diff;
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
