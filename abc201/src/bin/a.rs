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
        a1: i64, a2: i64, a3: i64
    };

    let v = vec![
        (a1, a2, a3),
        (a1, a3, a2),
        (a2, a1, a3),
        (a2, a3, a1),
        (a3, a1, a2),
        (a3, a2, a1)
    ];
    for (a1, a2, a3) in v {
        if a3 - a2 == a2 - a1 {
            println!("Yes");
            return;
        }
    }
    println!("No");
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
