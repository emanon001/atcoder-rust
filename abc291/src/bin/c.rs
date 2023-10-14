#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

/// 上下左右 (i, j)
pub const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn solve() {
    input! {
        _n: usize,
        s: Chars
    };

    let mut pos = (0_isize, 0_isize);
    let mut pos_set = HashSet::new();
    for ch in s {
        pos_set.insert(pos);
        pos = match ch {
            'R' => (pos.0 + 1, pos.1),
            'L' => (pos.0 - 1, pos.1),
            'U' => (pos.0, pos.1 + 1),
            'D' => (pos.0, pos.1 - 1),
            _ => unreachable!()
        };
        if pos_set.contains(&pos) {
            println!("Yes");
            return;
        }
    }
    println!("No");
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
