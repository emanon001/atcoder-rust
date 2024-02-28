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
        N: u64,
    };

    let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect::<Vec<_>>();
    let ans = convert_radix(N, 36)
        .into_iter()
        .map(|x| chars[x as usize])
        .join("");
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
