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
        A: i64, B: i64, C: i64, D: i64,
    };

    let x = Rational64::new(A, B);
    let y = Rational64::new(C, D);
    let ans = match x.cmp(&y) {
        std::cmp::Ordering::Less => "<",
        std::cmp::Ordering::Equal => "=",
        std::cmp::Ordering::Greater => ">",
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
