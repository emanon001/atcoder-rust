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
        T: Chars
    };

    let s1 = S[0] as u8 as isize;
    let s2 = S[1] as u8 as isize;
    let t1 = T[0] as u8 as isize;
    let t2 = T[1] as u8 as isize;

    let d1 = (s1 - s2).abs().min(5 - (s1 - s2).abs());
    let d2 = (t1 - t2).abs().min(5 - (t1 - t2).abs());

    let ans = if d1 == d2 { "Yes" } else { "No" };
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
