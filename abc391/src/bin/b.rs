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
        N: usize, M: usize,
        S: [Chars; N],
        T: [Chars; M],
    };

    for i in 0..N - M + 1 {
        for j in 0..N - M + 1 {
            let mut ok = true;
            for k in 0..M {
                for l in 0..M {
                    if S[i + k][j + l] != T[k][l] {
                        ok = false;
                    }
                }
            }
            if ok {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
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
