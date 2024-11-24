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
        S: Chars,
    };

    if N.is_even() {
        println!("No");
        return;
    }

    for i in 0..N / 2 {
        if S[i] != '1' {
            println!("No");
            return;
        }
    }
    if S[N / 2] != '/' {
        println!("No");
        return;
    }
    for i in (N / 2 + 1)..N {
        if S[i] != '2' {
            println!("No");
            return;
        }
    }
    println!("Yes");
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
