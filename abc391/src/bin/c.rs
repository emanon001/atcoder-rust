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

    let mut hole_to_p_count = vec![1; N];
    let mut p_to_hole = (0..N).collect::<Vec<_>>();
    let mut count = 0;
    for _ in 0..Q {
        input_interactive! {
            kind: usize,
        };
        match kind {
            1 => {
                input_interactive! {
                    p: Usize1, h: Usize1,
                };

                let cur_h = p_to_hole[p];
                p_to_hole[p] = h;
                hole_to_p_count[cur_h] -= 1;
                if hole_to_p_count[cur_h] == 1 {
                    count -= 1;
                }
                hole_to_p_count[h] += 1;
                if hole_to_p_count[h] == 2 {
                    count += 1;
                }
            }
            2 => {
                println!("{}", count);
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
