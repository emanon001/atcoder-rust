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
        n: usize, d: f64, h: f64,
        dhv: [(f64, f64); n]
    };
    let mut h1 = 0_f64;
    while h1 <= h {
        let mut is_ok = true;
        for &(d2, h2) in &dhv {
            if h1 >= h2 {
                continue;
            }
            // let x = (h - h1) / d;
            // let x = x / 2.0 * x;
            // let y = (h2 - h1) / d2;
            // let y = y / 2.0 * y;
            if (h - h1) / d < (h2 - h1) / d2 {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            println!("{}", h1);
            return;
        }
        h1 += 0.001_f64;
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
