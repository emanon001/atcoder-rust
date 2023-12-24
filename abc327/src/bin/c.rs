#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        grid: [[usize; 9]; 9]
    };

    for i in 0..9 {
        let mut set = HashSet::new();
        for j in 0..9 {
            set.insert(grid[i][j]);
        }
        if set.len() != 9 {
            println!("No");
            return;
        }
    }

    for j in 0..9 {
        let mut set = HashSet::new();
        for i in 0..9 {
            set.insert(grid[i][j]);
        }
        if set.len() != 9 {
            println!("No");
            return;
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            let mut set = HashSet::new();
            for add_i in 0..3 {
                for add_j in 0..3 {
                    set.insert(grid[i * 3 + add_i][j * 3 + add_j]);
                }
            }
            if set.len() != 9 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
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
