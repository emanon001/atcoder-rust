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
        n: i64
    };

    let mut res = BTreeSet::new();
    let len = n.to_string().len();
    for v in (0..=9).combinations_with_replacement(len) {
        let digit_sum = v.iter().sum::<i64>();
        let m = n - digit_sum;
        if m <= 0 {
            continue;
        }
        let m_digit_sum = m
            .to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as i64)
            .sum::<i64>();
        if m_digit_sum == digit_sum {
            res.insert(m);
        }
    }
    println!("{}", res.len());
    for x in res {
        println!("{}", x);
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
