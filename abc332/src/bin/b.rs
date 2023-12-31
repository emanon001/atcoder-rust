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
        K: isize, G: isize, M: isize,
    };

    let mut glass = 0;
    let mut mag = 0;
    for _ in 0..K {
        if glass == G {
            glass = 0;
        } else if mag == 0 {
            mag = M;
        } else {
            let diff = (G - glass).min(mag);
            glass += diff;
            mag -= diff;
        }
    }
    println!("{} {}", glass, mag);
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
