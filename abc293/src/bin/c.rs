#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input! {
        H: usize, W: usize,
        grid: [[usize; W]; H]
    };

    let STEP = H + W - 2;
    let mut ans = 0;
    for bits in 0..1 << STEP {
        let mut i = 0;
        let mut j = 0;
        let mut set = HashSet::new();
        set.insert(grid[i][j]);
        for s in 0..STEP {
            if (bits >> s & 1) == 0 {
                i += 1;
                if i >= H {
                    break;
                }
            } else {
                j += 1;
                if j >= W {
                    break;
                }
            }
            set.insert(grid[i][j]);
        }
        if i == H - 1 && j == W - 1 && set.len() == STEP + 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
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
