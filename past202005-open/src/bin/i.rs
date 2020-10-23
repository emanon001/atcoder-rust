#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn swap(v: &mut Vec<usize>, a: usize, b: usize) {
    let tmp = v[a];
    v[a] = v[b];
    v[b] = tmp;
}

fn solve() {
    input! {
        n: usize,
        q: usize
    };

    let mut row_pos = (0_usize..n).collect::<Vec<_>>();
    let mut col_pos = (0_usize..n).collect::<Vec<_>>();
    let mut is_transposed = false;
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    a: Usize1, b: Usize1
                };
                if is_transposed {
                    swap(&mut col_pos, a, b);
                } else {
                    swap(&mut row_pos, a, b);
                }
            }
            2 => {
                input! {
                    a: Usize1, b: Usize1
                };
                if is_transposed {
                    swap(&mut row_pos, a, b);
                } else {
                    swap(&mut col_pos, a, b);
                }
            }
            3 => {
                is_transposed = !is_transposed;
            }
            4 => {
                input! {
                    a: Usize1, b: Usize1
                };
                let (i, j) = if is_transposed {
                    let i = row_pos[b];
                    let j = col_pos[a];
                    (i, j)
                } else {
                    let i = row_pos[a];
                    let j = col_pos[b];
                    (i, j)
                };
                let res = i * n + j;
                println!("{}", res);
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
