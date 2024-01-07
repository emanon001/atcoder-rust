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
        N:  usize,
    };

    let mut ans = Vec::new();
    for x in 0..=21 {
        for y in 0..=21 {
            for z in 0..=21 {
                if x + y + z <= N {
                    ans.push((x, y, z));
                }
            }
        }
    }
    ans.sort();
    println!(
        "{}",
        ans.iter()
            .map(|(x, y, z)| format!("{x} {y} {z}"))
            .join("\n")
    );
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
