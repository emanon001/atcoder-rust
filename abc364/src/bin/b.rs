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
        H: usize, W: usize,
        S: (Usize1, Usize1),
        C: [Chars; H],
        X: Chars,
    };

    let mut pos = S;
    for x in X {
        match x {
            'L' => {
                if pos.1 > 0 && C[pos.0][pos.1 - 1] == '.' {
                    pos.1 -= 1;
                }
            }
            'R' => {
                if pos.1 < W - 1 && C[pos.0][pos.1 + 1] == '.' {
                    pos.1 += 1;
                }
            }
            'U' => {
                if pos.0 > 0 && C[pos.0 - 1][pos.1] == '.' {
                    pos.0 -= 1;
                }
            }
            'D' => {
                if pos.0 < H - 1 && C[pos.0 + 1][pos.1] == '.' {
                    pos.0 += 1;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{} {}", pos.0 + 1, pos.1 + 1);
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
