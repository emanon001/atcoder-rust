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
        n: usize,
        vv: [(isize, isize); n]
    };

    let sum = (0..n)
        .permutations(n)
        .map(|p| {
            p.windows(2)
                .map(|v| {
                    let i = v[0];
                    let j = v[1];
                    let (xi, yi) = vv[i];
                    let (xj, yj) = vv[j];
                    let m = ((xi - xj).pow(2) + (yi - yj).pow(2)) as f64;
                    m.sqrt()
                })
                .sum::<f64>()
        })
        .sum::<f64>();
    let res = sum / (1..=n).into_iter().fold(1, |acc, x| acc * x) as f64;
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
