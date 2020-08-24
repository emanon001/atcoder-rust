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
        ev: [usize; 6],
        b: usize,
        lv: [usize; 6]
    };

    let eset = ev.into_iter().collect::<HashSet<_>>();
    let lset = lv.into_iter().collect::<HashSet<_>>();
    let c = eset.intersection(&lset).count();
    let res = match c {
        6 => 1,
        5 => {
            let rest = *lset.difference(&eset).next().unwrap();
            if rest == b {
                2
            } else {
                3
            }
        }
        0..=2 => 0,
        n => 7 - n + 1,
    };
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
