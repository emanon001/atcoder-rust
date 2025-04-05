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
        N: u128, M: u32,
    };

    let mut ans = 0_u128;
    for i in 0..=M {
        match (N as u32).checked_pow(i) {
            Some(x) => {
                ans += x as u128;
                if ans > 1_000_000_000 {
                    println!("inf");
                    return;
                }
            }
            None => {
                println!("inf");
                return;
            }
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
