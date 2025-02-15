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
        S: String, S2: String,
    };

    let ans = match (S.as_str(), S2.as_str()) {
        ("sick", "sick") => 1,
        ("sick", "fine") => 2,
        ("fine", "sick") => 3,
        ("fine", "fine") => 4,
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
