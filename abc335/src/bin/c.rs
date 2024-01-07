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
    };
    let mut positions = Vec::new();
    for i in (1..=N).rev() {
        positions.push((i as isize, 0_isize));
    }
    let mut current_pos = positions[positions.len() - 1];
    for _ in 0..Q {
        input_interactive! {
            kind: usize
        };
        match kind {
            1 => {
                input_interactive! {
                    C: char
                };
                match C {
                    'R' => {
                        current_pos.0 += 1;
                    }
                    'L' => {
                        current_pos.0 -= 1;
                    }
                    'U' => {
                        current_pos.1 += 1;
                    }
                    'D' => {
                        current_pos.1 -= 1;
                    }
                    _ => unreachable!(),
                }
                positions.push(current_pos);
            }
            2 => {
                input_interactive! {
                    P: usize
                };
                let p = positions[positions.len() - P];
                println!("{} {}", p.0, p.1);
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
