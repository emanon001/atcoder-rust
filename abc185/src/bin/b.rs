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
        n: i64, m: usize, t: i64,
        abv: [(i64, i64); m]
    };

    let mut cur = n;
    let mut cur_t = 0;
    for (a, b) in abv {
        cur -= a - cur_t;
        if cur <= 0 {
            println!("No");
            return;
        }
        cur = (cur + (b - a)).min(n);
        cur_t = b;
    }
    cur -= t - cur_t;
    if cur <= 0 {
        println!("No");
    } else {
        println!("Yes");
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
