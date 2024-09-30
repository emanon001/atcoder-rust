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
        mut M: isize,
    };

    let mut ans = vec![];
    while M > 0 {
        let mut c = 0;
        let mut m = 1;
        while m * 3 <= M {
            m *= 3;
            c += 1;
        }
        ans.push(c);
        M -= m;
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
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
