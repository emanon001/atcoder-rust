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
        n: i64, a: i64, b: i64
    };
    let mut m = n;
    let mut is_ant = true;
    loop {
        m -= if is_ant { a } else { b };
        if m <= 0 {
            break;
        }
        is_ant = !is_ant;
    }
    let res = if is_ant { "Ant" } else { "Bug" };
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
