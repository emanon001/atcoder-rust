#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        P: [[f64; 6]; 3]
    };

    for k in 1..=18 {
        let mut ans = 0_f64;
        for a in 1..=6 {
            for b in 1..=6 {
                for c in 1..=6 {
                    if a + b + c == k {
                        ans +=
                            (P[0][a - 1] / 100.0) * (P[1][b - 1] / 100.0) * (P[2][c - 1] / 100.0);
                    }
                }
            }
        }
        println!("{}", ans);
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
