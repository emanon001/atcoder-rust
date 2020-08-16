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
        n: isize, r: isize,
        s: Chars
    };
    let mut cells = Vec::new();
    for i in (0..n).rev() {
        if s[i as usize] == '.' {
            cells.push(i);
        }
    }
    if cells.is_empty() {
        println!("0");
        return;
    }

    let mut time = std::cmp::max(0, cells[0] - r + 1);
    let mut l = n;
    for i in cells {
        if i >= l {
            continue;
        }
        time += 1;
        l = std::cmp::max(0, i - r + 1);
    }
    println!("{}", time);
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
