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
        s: String, t: String
    };

    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let res = match (s[0], t[0]) {
        ('B', 'B') => {
            (s[1].to_digit(10).unwrap() as isize - t[1].to_digit(10).unwrap() as isize).abs()
        }
        ('B', _) => {
            (-(s[1].to_digit(10).unwrap() as isize) - t[0].to_digit(10).unwrap() as isize).abs() - 1
        }
        (_, 'B') => {
            (s[0].to_digit(10).unwrap() as isize - -(t[1].to_digit(10).unwrap() as isize)).abs() - 1
        }
        _ => (s[0].to_digit(10).unwrap() as isize - t[0].to_digit(10).unwrap() as isize).abs(),
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
