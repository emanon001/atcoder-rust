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
        abcd: [i64; 4]
    };

    for bits in 0..1 << 4 {
        let mut cur = 0_i64;
        for i in 0..4 {
            if (bits >> i) & 1 == 1 {
                cur += abcd[i];
            } else {
                cur -= abcd[i];
            }
        }
        if cur == 0 {
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
