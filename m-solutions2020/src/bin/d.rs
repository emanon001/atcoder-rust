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
        av: [usize; n]
    };

    let mut x = av[0];
    let mut y = av[0];
    let mut pairs = Vec::new();
    for &a in av.iter().skip(1).chain(vec![0].iter()) {
        if a > y {
            y = a;
        } else {
            pairs.push((x, y));
            x = a;
            y = a;
        }
    }
    let pairs = pairs
        .into_iter()
        .filter(|(x, y)| x != y)
        .collect::<Vec<_>>();

    let mut money = 1000;
    for (x, y) in pairs {
        let v = money / x * y;
        let rest = money - (money / x * x);
        if v + rest > money {
            money = v + rest;
        }
    }
    println!("{}", money);
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
