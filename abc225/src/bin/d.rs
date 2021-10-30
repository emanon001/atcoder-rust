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

    let mut next_pos = vec![-1; n];
    let mut prev_pos = vec![-1; n];
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                };
                next_pos[x] = y as isize;
                prev_pos[y] = x as isize;
            }
            2 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                };
                next_pos[x] = -1;
                prev_pos[y] = -1;
            }
            3 => {
                input! {
                    x: Usize1,
                };
                let mut pos = x;
                while prev_pos[pos] != -1 {
                    pos = prev_pos[pos] as usize;
                }
                let mut res = Vec::new();
                res.push(pos + 1);
                while next_pos[pos] != -1 {
                    pos = next_pos[pos] as usize;
                    res.push(pos + 1);
                }
                let len = res.len();
                println!("{} {}", len, res.into_iter().join(" "));
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
