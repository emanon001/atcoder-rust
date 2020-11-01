#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f(digits: Vec<usize>) -> bool {
    let len = digits.len();
    let mut counts = vec![0; 10];
    for d in digits {
        counts[d] += 1;
    }
    let mut v = Vec::new();
    for d in 1..=9 {
        for _ in 0..(counts[d].min(3)) {
            v.push(d);
        }
    }
    for x in v.into_iter().permutations(len.min(3)) {
        let num = x.into_iter().fold(0, |acc, x| acc * 10 + x);
        if num % 2 != 0 {
            continue;
        }
        let a = num / 2;
        let b = a % 100;
        if b % 4 == 0 {
            return true;
        }
    }
    false
}

// fn f2(digits: Vec<usize>) -> bool {
//     for x in digits.into_iter().permutations(3) {
//         let num = x.into_iter().fold(0, |acc, x| acc * 10 + x);
//         if num % 8 == 0 {
//             return true;
//         }
//     }
//     false
// }

fn solve() {
    input! {
        s: Chars
    };

    let digits = s
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let res = if f(digits) { "Yes" } else { "No" };
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
