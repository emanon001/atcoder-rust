#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

use whiteread;

fn solve() {
    let n: usize = whiteread::parse_line().unwrap();
    let mut candidates = (1..=n * 2 + 1).collect::<BTreeSet<usize>>();
    loop {
        let m = *candidates.iter().next().unwrap();
        println!("{}", m);
        candidates.remove(&m);
        let m: usize = whiteread::parse_line().unwrap();
        if m == 0 {
            return;
        }
        candidates.remove(&m);
    }
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
