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
        n: usize, _: usize,
        avv: [[usize]; n],
        p: usize, q: usize,
        bv: [usize; p]
    };

    let mut res = 0;
    for av in avv {
        let mut c = 0;
        for a in av {
            for &b in &bv {
                if a == b {
                    c += 1;
                }
            }
        }
        if c >= q {
            res += 1;
        }
    }
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
