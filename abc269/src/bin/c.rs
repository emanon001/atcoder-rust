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
    };
    let binary = format!("{:b}", n).chars().collect::<Vec<_>>();
    let mut binary_one_pos = Vec::new();
    for i in 0..binary.len() {
        if binary[binary.len() - i - 1] == '1' {
            binary_one_pos.push(i);
        }
    }
    let mut res = Vec::new();
    for bits in 0..(1 << binary_one_pos.len()) {
        let mut m = 0_u64;
        for (i, pos) in binary_one_pos.iter().enumerate() {
            if (bits >> i) & 1 == 1 {
                m += 2.pow(*pos as u32);
            }
        }
        res.push(m);
    }
    res.sort();
    println!("{}", res.iter().join("\n"));
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
