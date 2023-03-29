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
        _: usize,
        t: Chars
    };

    let dirs = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];

    let mut pos: (isize, isize) = (0, 0);
    let mut dir = 0;
    for ch in t {
        if ch == 'S' {
            let d = dirs[dir];
            pos = (pos.0 + d.0, pos.1 + d.1);
        } else {
            dir = (dir + 1) % dirs.len();
        }
    }
    println!("{} {}", pos.0, pos.1);
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
