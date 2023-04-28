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
        n: usize, w: usize,
        av: [usize; n]
    };

    let mut res = HashSet::new();
    for i in 0..n {
        if av[i] <= w {
            res.insert(av[i]);
        }
        for j in i + 1..n {
            if av[i] + av[j] <= w {
                res.insert(av[i] + av[j]);
            }
            for k in j + 1..n {
                if av[i] + av[j] + av[k] <= w {
                    res.insert(av[i] + av[j] + av[k]);
                }
            }
        }
    }
    println!("{}", res.len());
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
