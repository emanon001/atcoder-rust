#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize, q: usize,
    };

    let mut vec = (1..=n).into_iter().collect::<Vec<_>>();
    let mut reversed = false;
    let to_i = |x: usize, reversed: bool| -> usize {
        if reversed {
            n - x - 1
        } else {
            x
        }
    };
    for _ in 0..q {
        input! {
            kind: usize,
        }
        match kind {
            1 => {
                input! {
                    x: Usize1, y: usize,
                };
                let i = to_i(x, reversed);
                vec[i] = y;
            }
            2 => {
                reversed = !reversed;
            }
            3 => {
                input! {
                    x: Usize1,
                };
                let i = to_i(x, reversed);
                println!("{}", vec[i]);
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
