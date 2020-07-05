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
        h: usize, w: usize, k: usize,
        grid: [Chars; h]
    };

    let mut res = 0;
    for hbits in 0..1 << h {
        for wbits in 0..1 << w {
            let mut count = 0;
            for h in 0..h {
                let hng = (hbits >> h) & 1 == 1;
                for w in 0..w {
                    let wng = (wbits >> w) & 1 == 1;
                    if !hng && !wng && grid[h][w] == '#' {
                        count += 1;
                    }
                }
            }
            if count == k {
                res += 1;
            }
        }
    }
    println!("{}", res);
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
