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
        av: [u64; n]
    };

    let mut map = HashMap::new();
    for &a in &av {
        *map.entry(a % 100).or_insert(0_u64) += 1;
    }
    let mut res = 0_u64;
    for &a in &av {
        let x = a % 100;
        *map.entry(x).or_insert(0_u64) -= 1;
        res += *map.get(&((100 - x) % 100)).unwrap_or(&0);
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
