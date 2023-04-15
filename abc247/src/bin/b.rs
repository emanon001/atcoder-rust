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
        stv: [(String, String); n]
    };

    let mut map = HashMap::new();
    for (s, t) in &stv {
        *map.entry(s).or_insert(0) += 1;
        if s != t {
            *map.entry(t).or_insert(0) += 1;
        }
    }
    for (s, t) in &stv {
        if map[s] > 1 && map[t] > 1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
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
