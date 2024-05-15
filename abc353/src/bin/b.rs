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
        N: usize, K: usize,
        A: [usize; N],
    };

    let mut ans = 0;
    let mut count = 0;
    for i in 0..N {
        if count + A[i] <= K {
            count += A[i];
        } else {
            ans += 1;
            count = A[i];
        }
    }
    if count > 0 {
        ans += 1;
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
