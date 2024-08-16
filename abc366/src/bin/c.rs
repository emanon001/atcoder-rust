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
        Q: usize,
    };

    let mut counts = HashMap::new();
    for _ in 0..Q {
        input_interactive! {
            kind: usize
        };
        match kind {
            1 => {
                input_interactive! {
                    x: usize,
                };
                *counts.entry(x).or_insert(0) += 1;
            }
            2 => {
                input_interactive! {
                    x: usize,
                };
                let entry = counts.entry(x).or_insert(0);
                *entry -= 1;
                if *entry == 0 {
                    counts.remove(&x);
                }
            }
            3 => {
                println!("{}", counts.len());
            }
            _ => unreachable!(),
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
