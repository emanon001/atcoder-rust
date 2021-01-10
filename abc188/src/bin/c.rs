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
        av: [i64; 2.pow(n as u32)]
    };
    let mut v = (0..(2.pow(n as u32) as usize)).collect::<Vec<_>>();
    while v.len() > 2 {
        let mut newv = Vec::new();
        for i in 0..v.len() / 2 {
            if av[v[2 * i]] > av[v[2 * i + 1]] {
                newv.push(v[2 * i]);
            } else {
                newv.push(v[2 * i + 1]);
            }
        }
        v = newv;
    }
    let res = if av[v[0]] > av[v[1]] { v[1] } else { v[0] } + 1;
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
