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
        S: Chars, T: String,
    };

    for w in 1..S.len() {
        let v = S
            .iter()
            .chunks(w)
            .into_iter()
            .map(|chunk| chunk.into_iter().collect::<Vec<_>>())
            .collect::<Vec<Vec<&char>>>();
        for i in 0..w {
            let s = v
                .iter()
                .map(|chunk| {
                    if i < chunk.len() {
                        chunk[i].to_string()
                    } else {
                        "".to_owned()
                    }
                })
                .join("");
            if s == T {
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
