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
        n: usize,
        av: [usize; n],
        bv: [usize; n],
        cv: [usize; n],
    };

    let mut ac = vec![0; 46];
    for a in av {
        ac[a % 46] += 1;
    }
    let mut bc = vec![0; 46];
    for b in bv {
        bc[b % 46] += 1;
    }
    let mut cc = vec![0; 46];
    for c in cv {
        cc[c % 46] += 1;
    }

    let mut res = 0_i64;
    for a in 0..ac.len() {
        for b in 0..bc.len() {
            let c = (46 - ((a + b) % 46)) % 46;
            res += ac[a] * bc[b] * cc[c];
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
