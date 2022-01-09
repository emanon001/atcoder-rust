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
        l: Usize1, r: Usize1,
        s: Chars,
    };

    let res = (&s[..l])
        .into_iter()
        .chain((&s[l..=r]).into_iter().rev())
        .chain((&s[r + 1..]).into_iter());
    println!("{}", res.into_iter().join(""));
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
