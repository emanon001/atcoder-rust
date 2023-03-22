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
        av: [i64; n]
    };

    let mut v = Vec::new();
    let mut count = 0_i64;
    for a in av {
        if let Some((pa, pc)) = v.pop() {
            if pa == a {
                if pc + 1 == a {
                    // already pop
                    count -= pc;
                } else {
                    v.push((a, pc + 1));
                    count += 1;
                }
            } else {
                v.push((pa, pc));
                v.push((a, 1));
                count += 1;
            }
        } else {
            v.push((a, 1));
            count = 1;
        }
        println!("{}", count);
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
