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

    let ans = if S.iter().filter(|ch| ch.is_uppercase()).count()
        > S.iter().filter(|ch| ch.is_lowercase()).count()
    {
        S.into_iter().map(|ch| ch.to_uppercase()).join("")
    } else {
        S.into_iter().map(|ch| ch.to_lowercase()).join("")
    };
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
