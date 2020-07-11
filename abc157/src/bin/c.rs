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
        n: usize, m: usize,
        scv: [(Usize1, char); m]
    };

    let mut res = std::usize::MAX;
    for x in 0..10.pow(n as u32) {
        let digits = x.to_string().chars().collect::<Vec<char>>();
        if digits.len() != n {
            continue;
        }
        let is_ok = scv.iter().all(|&(s, c)| s < digits.len() && digits[s] == c);
        if is_ok && x < res {
            res = x;
        }
    }
    let res = if res == std::usize::MAX {
        -1
    } else {
        res as isize
    };
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
