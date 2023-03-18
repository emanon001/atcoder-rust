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

    let mut s = s.into_iter().collect::<VecDeque<_>>();
    let front_a_count = s.iter().take_while(|&c| c == &'a').count();
    let back_a_count = s.iter().rev().take_while(|&c| c == &'a').count();
    if front_a_count > back_a_count {
        println!("No");
        return;
    }
    for _ in 0..(back_a_count - front_a_count) {
        s.push_front('a');
    }
    let res = if s.iter().join("") == s.iter().rev().join("") {
        "Yes"
    } else {
        "No"
    };
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
