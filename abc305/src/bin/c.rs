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

    let mut cookie_count = BTreeMap::new();
    for i in 0..h {
        let mut c = 0;
        for j in 0..w {
            if grid[i][j] == '#' {
                c += 1;
            }
        }
        if c > 0 {
            cookie_count.insert(c, i + 1);
        }
    }
    let mut cookie_count2 = BTreeMap::new();
    for j in 0..w {
        let mut c = 0;
        for i in 0..h {
            if grid[i][j] == '#' {
                c += 1;
            }
        }
        if c > 0 {
            cookie_count2.insert(c, j + 1);
        }
    }

    let i = cookie_count.iter().next().unwrap().1;
    let j = cookie_count2.iter().next().unwrap().1;
    println!("{} {}", i, j);
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
