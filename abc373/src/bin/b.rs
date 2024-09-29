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
        S: Chars,
    };

    let mut ans = 0;
    let mut pos = S.iter().position(|&c| c == 'A').unwrap();
    for ch in "BCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        let pos2 = S.iter().position(|&c| c == ch).unwrap();
        ans += (pos as isize - pos2 as isize).abs();
        pos = pos2;
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
