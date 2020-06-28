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
        d: usize,
        cv: [isize; 26],
        sv: [[isize; 26]; d],
        tv: [Usize1; d]
    };

    let mut last = vec![0; 26];
    let mut res = 0;
    for i in 0..d {
        let t = tv[i];
        res += sv[i][t];
        last[t] = i + 1;
        for j in 0..26 {
            res -= cv[j] * (i + 1 - last[j]) as isize;
        }
        println!("{}", res);
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
