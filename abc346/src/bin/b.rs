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
        W: usize, B: usize,
    };

    let s = "wbwbwwbwbwbw".repeat(200).chars().collect_vec();
    for i in 0..s.len() - W - B {
        let mut w_c = 0;
        let mut b_c = 0;
        for j in i..i + W + B {
            if s[j] == 'w' {
                w_c += 1;
            } else {
                b_c += 1;
            }
        }
        if w_c == W && b_c == B {
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
