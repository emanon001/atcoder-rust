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
        RGB: [usize; 3],
        C: String
    };

    let ans = match C.as_ref() {
        "Red" => RGB[1].min(RGB[2]),
        "Green" => RGB[0].min(RGB[2]),
        "Blue" => RGB[0].min(RGB[1]),
        _ => unreachable!(),
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
