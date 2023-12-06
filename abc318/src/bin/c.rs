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
        n: usize, d: usize, p: usize,
        mut f: [usize; n]
    };

    f.sort_by(|a, b| b.cmp(a));
    let mut ans = 0;
    for chunk in f.chunks(d) {
        let sum = chunk.iter().sum::<usize>();
        ans += sum.min(p);
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
