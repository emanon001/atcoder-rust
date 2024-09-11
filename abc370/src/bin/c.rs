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
        T: Chars,
    };
    let len = S.len();
    let mut ans = vec![];
    let mut s = S;
    while s != T {
        let mut v = vec![];
        for i in 0..len {
            if s[i] != T[i] {
                let mut new_s = s.clone();
                new_s[i] = T[i];
                v.push(new_s);
            }
        }
        v.sort();
        s = v[0].clone();
        ans.push(s.clone());
    }
    println!("{}", ans.len());
    if ans.len() > 0 {
        for v in ans {
            println!("{}", v.iter().join(""));
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
