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

    if s.len() != 8 {
        println!("No");
        return;
    }

    if !s[0].is_ascii_uppercase() {
        println!("No");
        return;
    }

    let ns = s[1..7].iter().join("").parse::<usize>();
    if let Ok(n) = ns {
        if n.to_string().len() != 6 {
            println!("No");
            return;
        }
    } else {
        println!("No");
        return;
    }

    if !s[7].is_ascii_uppercase() {
        println!("No");
        return;
    }

    println!("Yes");
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
