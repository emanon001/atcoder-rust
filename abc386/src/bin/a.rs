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
        A: usize, B: usize, C: usize, D: usize,
    };

    for e in 1..=13 {
        let cards = vec![A, B, C, D, e];
        if is_full_house(&cards) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn is_full_house(cards: &[usize]) -> bool {
    assert!(cards.len() == 5);
    let counts = cards.into_iter().counts();
    if counts.len() != 2 {
        return false;
    }
    let c = *counts.iter().next().unwrap().1;
    c == 2 || c == 3
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
