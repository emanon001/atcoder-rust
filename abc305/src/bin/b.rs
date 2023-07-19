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
        p: char, q: char
    };

    let l = p.min(q);
    let r = p.max(q);
    let mut res = 0;
    for (ch, d) in &[
        ('A', 3),
        ('B', 1),
        ('C', 4),
        ('D', 1),
        ('E', 5),
        ('F', 9),
        ('G', 0),
    ] {
        if (l..r).contains(ch) {
            res += d;
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
