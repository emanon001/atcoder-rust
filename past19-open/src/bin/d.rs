#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, T: isize,
        AB: [(isize, isize); N],
    };

    let (ta, tb) = AB
        .iter()
        .cloned()
        .sorted_by_key(|(a, b)| (std::cmp::Reverse(*a), *b))
        .collect::<Vec<_>>()[0];
    for (a, b) in AB {
        let ans = T * (ta - a) + (b - tb);
        println!("{}", ans);
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
