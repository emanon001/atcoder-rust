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
        sv: [i64; n]
    };

    let mut res = 0;
    for s in sv {
        let mut ok = false;
        for a in 1..=500 {
            for b in 1..=500 {
                if 4 * a * b + 3 * a + 3 * b == s {
                    ok = true;
                }
            }
        }
        if !ok {
            res += 1;
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
