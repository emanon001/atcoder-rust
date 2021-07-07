#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn check(v: &[char], t: &[char]) -> bool {
    for i in 0..t.len() {
        if t[i] != v[i % 3] {
            return false;
        }
    }
    true
}

fn solve() {
    input! {
        n: usize,
        t: Chars
    };

    let base = 10_i64.pow(10);

    if n == 1 {
        if t[0] == '0' {
            println!("{}", base);
        } else {
            println!("{}", base * 2);
        }
    } else if n == 2 {
        if t[0] == '1' && t[1] == '1' {
            // 11
            println!("{}", base);
        } else if t[0] == '1' && t[1] == '0' {
            // 10
            println!("{}", base);
        } else if t[0] == '0' && t[1] == '1' {
            // 01
            println!("{}", base - 1);
        } else {
            println!("0");
        }
        return;
    } else {
        if t[0] == '1' && t[1] == '1' && t[2] == '0' {
            // 110
            let v = vec!['1', '1', '0'];
            if !check(&v, &t) {
                println!("0");
                return;
            }
            println!("{}", base - ((n - 1) / 3) as i64);
        } else if t[0] == '1' && t[1] == '0' && t[2] == '1' {
            // 101
            let v = vec!['1', '0', '1'];
            if !check(&v, &t) {
                println!("0");
                return;
            }
            println!("{}", base - ((n + 1 - 1) / 3) as i64);
        } else if t[0] == '0' && t[1] == '1' && t[2] == '1' {
            // 011
            let v = vec!['0', '1', '1'];
            if !check(&v, &t) {
                println!("0");
                return;
            }
            println!("{}", base - ((n + 2 - 1) / 3) as i64);
        } else {
            println!("0");
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
