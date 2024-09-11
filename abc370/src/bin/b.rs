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
    };
    let mut A = vec![];
    for i in 0..N {
        input_interactive! {
            a: [Usize1; i + 1],
        };
        A.push(a);
    }
    let mut i = 0;
    for j in 0..N {
        i = if i >= j { A[i][j] } else { A[j][i] };
    }
    let ans = i + 1;
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
