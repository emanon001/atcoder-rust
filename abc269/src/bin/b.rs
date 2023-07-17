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
        grid: [Chars; 10]
    };

    let mut a = 0;
    'outer: for i in 0..10 {
        for j in 0..10 {
            if grid[i][j] == '#' {
                a = i;
                break 'outer;
            }
        }
    }
    let mut b = 0;
    'outer: for i in (0..10).rev() {
        for j in 0..10 {
            if grid[i][j] == '#' {
                b = i;
                break 'outer;
            }
        }
    }
    let mut c = 0;
    'outer: for i in 0..10 {
        for j in 0..10 {
            if grid[i][j] == '#' {
                c = j;
                break 'outer;
            }
        }
    }
    let mut d = 0;
    'outer: for i in 0..10 {
        for j in (0..10).rev() {
            if grid[i][j] == '#' {
                d = j;
                break 'outer;
            }
        }
    }
    println!("{} {}", a + 1, b + 1);
    println!("{} {}", c + 1, d + 1);
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
