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
        N: usize, M: usize,
        A: [i64; N],
        B: [i64; M],
    };

    let C = A.iter().chain(&B).sorted().copied().collect::<Vec<i64>>();
    let a_set = A.iter().copied().collect::<HashSet<_>>();
    for i in 0..C.len() - 1 {
        if a_set.contains(&C[i]) && a_set.contains(&C[i + 1]) {
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
