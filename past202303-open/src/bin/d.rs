#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: i64, A: i64, B: i64, C: i64, D: i64
    };

    let mut ans = std::i64::MAX;
    for c in 0..40 {
        let mut h = H;
        let mut money = 0;
        for _ in 0..c {
            money += D;
            h -= C;
            if h > 0 {
                h -= h / 2;
            }
        }
        if h > 0 {
            money += (h + A - 1) / A * B;
        }
        chmin!(ans, money);
    }
    println!("{}", ans);
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
