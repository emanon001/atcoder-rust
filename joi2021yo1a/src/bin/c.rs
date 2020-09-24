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
        n: usize, m: usize,
        av: [usize; n],
        bv: [usize; m]
    };

    let a_set = av.into_iter().collect::<BTreeSet<_>>();
    let b_set = bv.into_iter().collect::<BTreeSet<_>>();
    for x in a_set.intersection(&b_set) {
        println!("{}", x);
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
