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
        N: Chars,
    };

    let N = N
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect::<Vec<_>>();
    for p in 0..N.len() {
        for q in 0..=9 {
            if p == 0 && q == 0 {
                continue;
            }
            let mut digits = N.clone();
            digits[p] = q;
            let is_ok = digits
                .into_iter()
                .tuple_windows()
                .all(|(a, b)| (a - b).abs() <= 1);
            if is_ok {
                println!("Yes");
                return;
            }
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
