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
        N: usize,
        A: [isize; N],
    };

    let mut ans_in = 0;
    let mut ans_out = 0;
    for i in 0..N - 1 {
        if A[i] < A[i + 1] {
            ans_in += A[i + 1] - A[i];
        } else {
            ans_out += A[i] - A[i + 1];
        }
    }
    println!("{} {}", ans_in, ans_out);
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
