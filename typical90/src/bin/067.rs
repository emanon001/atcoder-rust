#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn convert_radix(radix10: u64, base: u64) -> Vec<u64> {
    let mut x = radix10;
    let mut ans = Vec::new();
    loop {
        ans.push(x % base);
        x /= base;
        if x == 0 {
            break;
        }
    }
    ans.reverse();
    ans
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: String, K: usize,
    };

    let mut ans = N;
    for _ in 0..K {
        let base10 = u64::from_str_radix(&ans, 8).expect("valid base8");
        let base8 = convert_radix(base10, 9)
            .into_iter()
            .map(|x| if x == 8 { 5 } else { x })
            .join("");
        ans = base8;
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
