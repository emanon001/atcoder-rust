#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{stdin, BufReader};

fn solve() {
    let mut source = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut source, $($tt)*)));
    input! {
        _l: usize, n1: usize, n2: usize,
        vl1: [(i64, i64); n1],
        vl2: [(i64, i64); n2],
    };

    let mut ans = 0_i64;
    let mut current_pos = 0;
    let mut r1 = -1;
    let mut l2 = 0;
    let mut r2 = l2 + vl2[0].1 - 1;
    for (v, l) in vl1 {
        let l1 = r1 + 1;
        r1 = l1 + l - 1;
        loop {
            if current_pos >= n2 {
                break;
            }
            if v == vl2[current_pos].0 && is_overlapping(l1, r1, l2, r2) {
                ans += overlapping_size(l1, r1, l2, r2);
            }
            if r1 < r2 {
                break;
            }
            current_pos += 1;
            if current_pos >= n2 {
                break;
            }
            l2 = r2 + 1;
            r2 = l2 + vl2[current_pos].1 - 1;
        }
    }
    println!("{}", ans);
}

fn is_overlapping(l1: i64, r1: i64, l2: i64, r2: i64) -> bool {
    // l1, l2, r2, r1
    (l1 <= l2 && l2 <= r2 && r2 <= r1) ||
    // l1, l2, r1, r2
    (l1 <= l2 && l2 <= r1 && r1 <= r2) ||
    // l2, l1, r1, r2
    (l2 <= l1 && l1 <= r1 && r1 <= r2) ||
    // l2, l1, r2, r1
    (l2 <= l1 && l1 <= r2 && r2 <= r1)
}

fn overlapping_size(l1: i64, r1: i64, l2: i64, r2: i64) -> i64 {
    // l1, l2, r2, r1
    if l1 <= l2 && l2 <= r2 && r2 <= r1 {
        return r2 - l2 + 1;
    }
    // l1, l2, r1, r2
    if l1 <= l2 && l2 <= r1 && r1 <= r2 {
        return r1 - l2 + 1;
    }
    // l2, l1, r1, r2
    if l2 <= l1 && l1 <= r1 && r1 <= r2 {
        return r1 - l1 + 1;
    }
    // l2, l1, r2, r1
    if l2 <= l1 && l1 <= r2 && r2 <= r1 {
        return r2 - l1 + 1;
    }

    0
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
