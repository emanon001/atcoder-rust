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
        av: [isize; n]
    };

    let mut min = std::isize::MAX;
    let mut min_i = 0;
    for i in 0..n {
        let a = av[i];
        if a < min {
            min = a;
            min_i = i;
        }
    }
    let mut max = std::isize::MIN;
    let mut max_i = 0;
    for i in 0..n {
        let a = av[i];
        if a > max {
            max = a;
            max_i = i;
        }
    }
    if min.abs() > max.abs() {
        let mut res = Vec::new();
        res.push((min_i + 1, n));
        res.push((min_i + 1, n));
        for i in (0..n - 1).rev() {
            if av[i] > 0 {
                let i = i + 1;
                res.push((i + 1, i));
                res.push((i + 1, i));
            } else {
                let i = i + 1;
                res.push((i + 1, i));
            }
        }
        println!("{}", res.len());
        for (x, y) in res {
            println!("{} {}", x, y);
        }
    } else {
        let mut res = Vec::new();
        res.push((max_i + 1, 1));
        res.push((max_i + 1, 1));
        for i in 1..n {
            if av[i] > 0 {
                let i = i + 1;
                res.push((i - 1, i));
            } else {
                let i = i + 1;
                res.push((i - 1, i));
                res.push((i - 1, i));
            }
        }
        println!("{}", res.len());
        for (x, y) in res {
            println!("{} {}", x, y);
        }
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
