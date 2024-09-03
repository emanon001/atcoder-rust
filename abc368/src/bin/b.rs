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
        N: usize,
        mut A: [isize; N],
    };

    let mut ans = 0;
    while A.iter().filter(|&a| *a > 0).count() >= 2 {
        ans += 1;
        A.sort_by_key(|a| std::cmp::Reverse(*a));
        A[0] -= 1;
        A[1] -= 1;
    }
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
