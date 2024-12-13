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
        N: usize, M: usize,
    };

    let mut scores = vec![0_isize; N];
    let mut last_atack_players = vec![None; N];
    for _ in 0..M {
        input_interactive! {
            kind: usize,
        };
        match kind {
            1 => {
                input_interactive! {
                    x: Usize1,
                    y: Usize1,
                };
                last_atack_players[y] = Some(x);
            }
            2 => {
                input_interactive! {
                    z: Usize1,
                };
                scores[z] -= 1;
                if let Some(x) = last_atack_players[z] {
                    scores[x] += 1;
                    last_atack_players[z] = None;
                }
            }
            _ => unreachable!(),
        };
    }
    println!("{}", scores.iter().join(" "));
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
