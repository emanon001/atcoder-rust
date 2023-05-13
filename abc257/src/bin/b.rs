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
        n: usize, k: usize, q: usize,
        mut av: [usize; k],
        lv: [Usize1; q],
    };

    for l in lv {
        if av[l] == n {
            continue;
        }
        if l < k - 1 && av[l] + 1 == av[l + 1] {
            continue;
        }
        av[l] += 1;
    }
    println!("{}", av.iter().join(" "));
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
