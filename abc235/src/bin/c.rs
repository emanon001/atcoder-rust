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
        n: usize, q: usize,
        av: [usize; n],
        xkv: [(usize, usize); q]
    };

    let mut count_map = HashMap::new();
    let mut position_map = HashMap::new();
    for i in 0..n {
        let a = av[i];
        let c = count_map.entry(a).or_insert(0);
        *c += 1;
        let v = i + 1;
        position_map.insert((a, *c), v);
    }
    for xk in xkv {
        let res = if let Some(&v) = position_map.get(&xk) {
            v as isize
        } else {
            -1
        };
        println!("{}", res);
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
