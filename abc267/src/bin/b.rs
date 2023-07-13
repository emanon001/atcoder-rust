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
        s: Chars,
    };

    let mut v = vec![Vec::new(); 7];
    for (i, ch) in s.into_iter().enumerate() {
        let no = i + 1;
        let state = ch == '1';
        match no {
            7 => {
                v[0].push(state);
            }
            4 => {
                v[1].push(state);
            }
            2 | 8 => {
                v[2].push(state);
            }
            1 | 5 => {
                v[3].push(state);
            }
            3 | 9 => {
                v[4].push(state);
            }
            6 => {
                v[5].push(state);
            }
            10 => {
                v[6].push(state);
            }
            _ => unreachable!(),
        }
    }

    if v[3][0] {
        println!("No");
        return;
    }

    for i in 0..7 {
        for j in i + 2..7 {
            if !v[i].contains(&true) {
                continue;
            }
            if !v[j].contains(&true) {
                continue;
            }
            if (i + 1..j).any(|k| v[k].iter().all(|st| !st)) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
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
