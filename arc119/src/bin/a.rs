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
        n: i128
    };

    // let mut b = 0;
    // while 2.pow(b + 1) <= n {
    //     b += 1;
    // }
    let mut res = 1_i128 << 60;
    for b in 0..=60 {
        let a = n / 2.pow(b);
        let c = n - a * 2.pow(b);
        let x = a + b as i128 + c;
        chmin!(res, x);
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
