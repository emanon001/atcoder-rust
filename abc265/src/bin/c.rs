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
        h: usize, w: usize,
        grid: [Chars; h]
    };

    let mut pos = (0, 0);
    let mut visited = HashSet::new();
    loop {
        visited.insert(pos);
        match grid[pos.0][pos.1] {
            'U' => {
                if pos.0 == 0 {
                    println!("{} {}", pos.0 + 1, pos.1 + 1);
                    return;
                }
                pos = (pos.0 - 1, pos.1);
            }
            'D' => {
                if pos.0 == h - 1 {
                    println!("{} {}", pos.0 + 1, pos.1 + 1);
                    return;
                }
                pos = (pos.0 + 1, pos.1);
            }
            'L' => {
                if pos.1 == 0 {
                    println!("{} {}", pos.0 + 1, pos.1 + 1);
                    return;
                }
                pos = (pos.0, pos.1 - 1);
            }
            'R' => {
                if pos.1 == w - 1 {
                    println!("{} {}", pos.0 + 1, pos.1 + 1);
                    return;
                }
                pos = (pos.0, pos.1 + 1);
            }
            _ => unreachable!(),
        }
        if visited.contains(&pos) {
            println!("-1");
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
