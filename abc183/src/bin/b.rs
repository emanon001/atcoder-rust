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
        sx: i64, sy: i64, gx: i64, gy: i64
    };

    let a = sy as f64 / (sy + gy) as f64;
    if sx <= gx {
        let b = (gx - sx) as f64 * a;
        let res = sx as f64 + b;
        println!("{}", res);
    } else {
        let b = (sx - gx) as f64 * a;
        let res = sx as f64 - b;
        println!("{}", res);
    }

    // for x in -(10_i64.pow(6))..=10_i64.pow(6) {
    //     if x > sx && x > gx {
    //         continue;
    //     }
    //     if x < sx && x < gx {
    //         continue;
    //     }
    //     if x >= sx {
    //         let xd1 = (x - sx) as f64;
    //         let yd1 = sy as f64;
    //         let xd2 = (gx - x) as f64;
    //         let yd2 = gy as f64;
    //         if yd1 / xd1 == yd2 / xd2 {
    //             println!("{}", x);
    //             return;
    //         }
    //     }
    // }
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
