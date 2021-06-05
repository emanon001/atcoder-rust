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
        bv: [usize; m],
    };

    let aset = av.into_iter().collect::<BTreeSet<_>>();
    let bset = bv.into_iter().collect::<BTreeSet<_>>();
    let res = aset.symmetric_difference(&bset).join(" ");
    println!("{}", res);
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
