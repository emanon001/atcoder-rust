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
        wv: [Chars; n]
    };

    let mut turn = 1;
    let mut used = HashSet::new();
    let mut prev_suffix = wv[0][0];
    for w in wv {
        if used.contains(&w) {
            let res = if turn % 2 == 1 { "LOSE" } else { "WIN" };
            println!("{}", res);
            return;
        }
        if w[0] != prev_suffix {
            let res = if turn % 2 == 1 { "LOSE" } else { "WIN" };
            println!("{}", res);
            return;
        }
        prev_suffix = w[w.len() - 1];
        used.insert(w);
        turn += 1;
    }
    println!("DRAW");
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
