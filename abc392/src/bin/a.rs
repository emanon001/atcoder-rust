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
        A: [usize; 3],
    };

    for (a, b, c) in [
        (A[0], A[1], A[2]),
        (A[0], A[2], A[1]),
        (A[1], A[0], A[2]),
        (A[1], A[2], A[0]),
        (A[2], A[0], A[1]),
        (A[2], A[1], A[0]),
    ] {
        if a * b == c {
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
