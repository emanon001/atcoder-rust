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

    if S.len().is_odd() {
        println!("No");
        return;
    }

    let mut set = HashSet::new();
    for i in 0..S.len() / 2 {
        if S[2 * i] != S[2 * i + 1] {
            println!("No");
            return;
        }
        if set.contains(&S[2 * i]) {
            println!("No");
            return;
        }
        set.insert(S[2 * i]);
    }
    println!("Yes");
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
