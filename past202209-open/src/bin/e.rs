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
        R: isize, N: isize, M: isize, L: isize,
        S: [isize; L]
    };

    let mut round = 1;
    let mut count = 0;
    let mut sum = 0;
    for s in S {
        count += 1;
        sum += s;

        if sum > N {
            println!("No");
            return;
        }

        if sum == N || count == M {
            sum = 0;
            count = 0;
            round += 1;
        }
    }
    if round != R + 1 {
        println!("No");
        return;
    }
    let ans = if count == 0 { "Yes" } else { "No" };
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
