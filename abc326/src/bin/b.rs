#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        n: usize
    };

    for n in n.. {
        let n = n
            .to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as isize)
            .collect::<Vec<_>>();
        if n[0] * n[1] == n[2] {
            println!("{}", n.into_iter().join(""));
            return;
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
