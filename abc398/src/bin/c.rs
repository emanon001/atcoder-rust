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
        A: [usize; N],
    };

    let max = A
        .iter()
        .counts()
        .iter()
        .filter(|(_, &c)| c == 1)
        .map(|(&a, _)| *a)
        .max();
    match max {
        Some(max) => {
            for (i, a) in A.into_iter().enumerate() {
                let i = i + 1;
                if a == max {
                    println!("{}", i);
                    return;
                }
            }
        }
        None => {
            println!("-1");
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
