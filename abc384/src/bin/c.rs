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
        ABCDE: [u64; 5],
    };

    let mut ans = Vec::new();
    for bits in 1..(1 << ABCDE.len()) {
        let mut name = Vec::new();
        let mut score = 0;
        for i in 0..ABCDE.len() {
            if (bits >> i) & 1 == 1 {
                name.push(('A' as u8 + i as u8) as char);
                score += ABCDE[i];
            }
        }
        ans.push((score, name.into_iter().join("")));
    }
    ans.sort_by_key(|(s, n)| (std::cmp::Reverse(*s), n.clone()));
    println!("{}", ans.into_iter().map(|(_, n)| n).join("\n"));
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
