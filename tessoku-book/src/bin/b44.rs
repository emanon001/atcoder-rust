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
        n: usize,
        grid: [[usize; n]; n],
        q: usize
    };

    let mut row_pos = (0..n).collect::<Vec<_>>();
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    x: Usize1, y: Usize1
                };
                row_pos.swap(x, y);
            }
            2 => {
                input! {
                    x: Usize1, y: Usize1
                };
                println!("{}", grid[row_pos[x]][y]);
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
