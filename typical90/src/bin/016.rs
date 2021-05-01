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
        n: i64,
        a: i64, b: i64, c: i64
    };

    let max_coins = 9999;
    let mut res = i64::max_value();
    for x in 0..=max_coins {
        for y in 0..=max_coins - x {
            if a * x + b * y > n {
                break;
            }
            let rest = n - a * x - b * y;
            if rest % c != 0 {
                continue;
            }
            let count = x + y + rest / c;
            if count > max_coins {
                continue;
            }
            chmin!(res, count);
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
