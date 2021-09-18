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
        x: Chars,
        n: usize,
        mut sv: [Chars; n]
    };

    let mut order: HashMap<char, usize> = HashMap::new();
    for i in 0..26 {
        order.insert(x[i], i);
    }
    sv.sort_by_key(|s| {
        let s = s.clone();
        s.into_iter().map(|ch| order[&ch]).collect::<Vec<_>>()
    });

    for s in sv {
        println!("{}", s.iter().join(""));
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
