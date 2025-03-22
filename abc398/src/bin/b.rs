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
        A: [Usize1; 7]
    };

    for comb in A.into_iter().combinations(5) {
        let counts = comb.iter().counts();
        if counts.len() == 2 && counts.values().any(|&c| c == 2 || c == 3) {
            println!("Yes");
            return;
        }
    }

    println!("No");
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
