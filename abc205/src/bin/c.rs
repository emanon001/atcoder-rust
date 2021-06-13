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
        a: i64, b: i64, c: i64
    };

    if a == b {
        println!("=");
        return;
    }

    if a.abs() == b.abs() && c % 2 == 0 {
        println!("=");
        return;
    }

    if a >= 0 && b >= 0 {
        let res = if a > b {
            ">"
        } else {
            "<"
        };
        println!("{}", res);
        return;
    }

    if a <= 0 && b <= 0 {
        if c % 2 == 0 {
            if a < b {
                println!(">")
            } else {
                println!("<")
            }
        } else {
            if a < b {
                println!("<")
            } else {
                println!(">")
            }
        }
        return;
    }

    if c % 2 == 0 {
        if a.abs() > b.abs() {
            println!(">")
        } else {
            println!("<")
        }
    } else {
        if a > b {
            println!(">")
        } else {
            println!("<")
        }
    }
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
