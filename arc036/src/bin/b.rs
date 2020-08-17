#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[derive(PartialEq, Eq)]
enum Mode {
    Up,
    Down,
}

fn solve() {
    input! {
        n: usize,
        hv: [usize; n]
    };

    let mut res = 0;
    let mut mode = Mode::Up;
    let mut s = 0;
    let mut prev = 0;
    for i in 0..n {
        let h = hv[i];
        match mode {
            Mode::Up => {
                if prev > h {
                    mode = Mode::Down;
                }
            }
            Mode::Down => {
                if prev < h {
                    mode = Mode::Up;
                    s = i - 1;
                }
            }
        }
        let score = i - s + 1;
        if score > res {
            res = score;
        }
        prev = h;
    }
    println!("{}", res);
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
