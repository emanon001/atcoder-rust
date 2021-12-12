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
        sv: [String; n]
    };

    let mut map = HashMap::new();
    for s in sv {
        *map.entry(s).or_insert(0_i32) += 1;
    }
    let mut res = "".to_string();
    let mut count = 0;
    for (s, c) in map {
        if c >= count {
            count = c;
            res = s;
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
