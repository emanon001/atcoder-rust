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
        _: usize,
        S: Chars,
    };

    let zero_count = S.iter().filter(|&c| c == &'0').count();
    let ans = if zero_count == 1 {
        "-1".to_string()
    } else {
        let mut zero_values = vec![];
        for (i, ch) in S.iter().enumerate() {
            if ch == &'0' {
                zero_values.push(i + 1);
            }
        }
        let mut zero_i = 0;
        S.iter()
            .enumerate()
            .map(|(i, ch)| {
                if ch == &'0' {
                    let res = zero_values[(zero_i + 1) % zero_values.len()].to_string();
                    zero_i += 1;
                    res
                } else {
                    (i + 1).to_string()
                }
            })
            .join(" ")
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
