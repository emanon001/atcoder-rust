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
        n: usize, k: usize,
        av: [i64; n]
    };

    let mut map = HashMap::new();
    for a in av {
        *map.entry(a).or_insert(0) += 1;
    }
    let mut res = 0;
    for i in 1..=k {
        for x in 0..=n as i64 {
            if map.get(&x).unwrap_or(&0) < &i {
                res += x;
                break;
            }
        }
    }
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
