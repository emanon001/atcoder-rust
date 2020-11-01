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
        xyv: [(f64, f64); n]
    };

    for a in 0..n {
        let axy = xyv[a];
        for b in a + 1..n {
            let bxy = xyv[b];
            for c in b + 1..n {
                let cxy = xyv[c];
                // a - b - c
                let v1 = (bxy.1 - axy.1) / (bxy.0 - axy.0);
                let v2 = (cxy.1 - bxy.1) / (cxy.0 - bxy.0);
                if v1 == v2 {
                    println!("Yes");
                    return;
                }
                // a - c - b
                let v1 = (cxy.1 - axy.1) / (cxy.0 - axy.0);
                let v2 = (bxy.1 - cxy.1) / (bxy.0 - cxy.0);
                if v1 == v2 {
                    println!("Yes");
                    return;
                }
                // b - a - c
                let v1 = (axy.1 - bxy.1) / (axy.0 - bxy.0);
                let v2 = (cxy.1 - axy.1) / (cxy.0 - axy.0);
                if v1 == v2 {
                    println!("Yes");
                    return;
                }
                // b - c - a
                let v1 = (cxy.1 - bxy.1) / (cxy.0 - bxy.0);
                let v2 = (axy.1 - cxy.1) / (axy.0 - cxy.0);
                if v1 == v2 {
                    println!("Yes");
                    return;
                }
                // c - a - b
                let v1 = (axy.1 - cxy.1) / (axy.0 - cxy.0);
                let v2 = (bxy.1 - axy.1) / (bxy.0 - axy.0);
                if v1 == v2 {
                    println!("Yes");
                    return;
                }
                // c - b - a
                let v1 = (bxy.1 - cxy.1) / (bxy.0 - cxy.0);
                let v2 = (axy.1 - bxy.1) / (axy.0 - bxy.0);
                if v1 == v2 {
                    println!("Yes");
                    return;
                }
            }
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
