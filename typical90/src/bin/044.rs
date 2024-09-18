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
        N: usize, Q: usize,
        mut A: [i64; N],
        TXY: [(usize, usize, usize); Q],
    };

    let mut shift = 0;
    for (t, x, y) in TXY {
        match t {
            1 => {
                let i = pos(x - 1, shift, N);
                let j = pos(y - 1, shift, N);
                A.swap(i, j);
            }
            2 => {
                shift = (shift + 1) % N;
            }
            3 => {
                let i = pos(x - 1, shift, N);
                println!("{}", A[i]);
            }
            _ => unreachable!(),
        }
    }
}

fn pos(x: usize, shift: usize, n: usize) -> usize {
    if x >= shift {
        x - shift
    } else {
        n - (shift - x)
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
