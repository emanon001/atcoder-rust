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
        N: usize, R: isize,
        DA: [(usize, isize); N],
    };

    let mut ans = R;
    for (d, a) in DA {
        match d {
            1 => {
                if (1600..=2799).contains(&ans) {
                    ans += a;
                }
            }
            2 => {
                if (1200..=2399).contains(&ans) {
                    ans += a;
                }
            }
            _ => unreachable!(),
        }
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
