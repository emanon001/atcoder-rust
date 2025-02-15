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
        S: Chars,
    };

    let mut ans = 0;
    for i in 0..S.len() {
        for j in i + 1..S.len() {
            for k in j + 1..S.len() {
                if j - i == k - j && S[i] == 'A' && S[j] == 'B' && S[k] == 'C' {
                    ans += 1;
                }
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
