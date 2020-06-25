#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        s: Chars
    };

    let odd_chars = vec!['R', 'U', 'D'].into_iter().collect::<HashSet<_>>();
    let even_chars = vec!['L', 'U', 'D'].into_iter().collect::<HashSet<_>>();
    let is_ok = s.into_iter().enumerate().all(|(i, ch)| {
        let i = i + 1;
        if i % 2 == 0 {
            even_chars.contains(&ch)
        } else {
            odd_chars.contains(&ch)
        }
    });
    let res = if is_ok { "Yes" } else { "No" };
    println!("{}", res);
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
