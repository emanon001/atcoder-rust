#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn is_sections_overlapping(l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
    // l1, l2, r2, r1
    (l1 <= l2 && r2 <= r1)
        // l2, l1, r1, r2
        || (l2 <= l1 && r1 <= r2)
        // l1, l2, r1
        || (l1 <= l2 && l2 < r1)
        // l2, l1, r2
        || (l2 <= l1 && l1 < r2)
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        a: usize, b: usize, c: usize, d: usize, e: usize, f: usize,
        g: usize, h: usize, i: usize, j: usize, k: usize, l: usize,
    };

    let ans = if is_sections_overlapping(a, d, g, j)
        && is_sections_overlapping(b, e, h, k)
        && is_sections_overlapping(c, f, i, l)
    {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans)
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
