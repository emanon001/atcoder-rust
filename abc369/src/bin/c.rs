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
        A: [i64; N],
    };

    let mut ans = N;
    let mut l = 0;
    let mut r = l + 1;
    while r < N {
        let diff = A[l] - A[l + 1];
        if A[r - 1] - A[r] == diff {
            ans += r - l;
            r += 1;
        } else {
            l = r - 1;
            r = r;
        }
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
