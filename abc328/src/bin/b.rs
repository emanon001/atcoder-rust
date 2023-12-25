#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        n: usize,
        d: [usize; n]
    };

    let mut ans = 0;
    for (i, d_i) in d.into_iter().enumerate() {
        let month = i + 1;
        for day in 1..=d_i {
            let md = format!("{}{}", month, day);
            if md.chars().unique().count() == 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
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
