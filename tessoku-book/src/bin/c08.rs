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
        stv: [(usize, usize); n]
    };

    let mut res = Vec::new();
    for tosen in 0..10000 {
        let tosen = format!("{:0>4}", tosen).chars().collect::<Vec<_>>();
        let valid = stv.iter().all(|&(s, t)| {
            let s = format!("{:0>4}", s).chars().collect::<Vec<_>>();
            let mut same_count = 0;
            for i in 0..4 {
                same_count += if tosen[i] == s[i] { 1 } else { 0 };
            }
            (same_count == 4 && t == 1) || (same_count == 3 && t == 2) || (same_count < 3 && t == 3)
        });
        if valid {
            res.push(tosen.iter().join(""));
        }
    }
    if res.len() >= 2 {
        println!("Can't Solve");
    } else {
        println!("{}", res[0]);
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
