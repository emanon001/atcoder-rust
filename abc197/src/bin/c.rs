#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize,
        av: [u32; n]
    };

    let mut res = u32::max_value();
    for bits in 0..1 << (n - 1) {
        let bits = bits | 1 << (n - 1);
        let mut xor = 0;
        let mut or = 0;
        for i in 0..n {
            or |= av[i];
            if (bits >> i) & 1 == 1 {
                xor ^= or;
                or = 0;
            }
        }
        chmin!(res, xor);
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
