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
        N: usize, _: usize,
        S: Chars
    };

    let mut cards = vec![0; N];
    let mut bafuda = 0;
    for (i, ch) in S.into_iter().enumerate() {
        let i = i % N;
        cards[i] += 1;
        match ch {
            '+' => {
                cards[i] += bafuda;
                bafuda = 0;
            }
            '0' => {
                // do nothing
            }
            '-' => {
                bafuda += cards[i];
                cards[i] = 0;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", cards.iter().join("\n"));
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
