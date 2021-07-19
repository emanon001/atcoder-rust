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
        mut av: [i64; n]
    };

    let mut shift = 0_usize;
    for _ in 0..q {
        input! {
            t: usize
        };
        match t {
            1 => {
                input! {
                    x: Usize1, y: Usize1
                };
                let i = (n + x - shift) % n;
                let j = (n + y - shift) % n;
                let t = av[i];
                av[i] = av[j];
                av[j] = t;
            }
            2 => {
                input! {
                    _: usize, _: usize
                };
                shift = (shift + 1) % n;
            }
            3 => {
                input! {
                    x: Usize1, _: usize
                };
                let i = (n + x - shift) % n;
                println!("{}", av[i]);
            }
            _ => unreachable!()
        };
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
