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
        H: usize, W: usize,
        S: [String; H],
        T: [String; H],
    };

    let mut twice_s_and_t = vec![];
    for (s, t) in S.into_iter().zip(T.into_iter()) {
        if s.chars().unique().count() == 1 {
            if s != t {
                println!("No");
                return;
            }
        } else {
            twice_s_and_t.push((s.repeat(2), t));
        }
    }

    if twice_s_and_t.is_empty() {
        println!("Yes");
        return;
    }

    if let Some(pos) = twice_s_and_t[0].0.find(&twice_s_and_t[0].1) {
        let ok = twice_s_and_t.into_iter().all(|(s, t)| s[pos..pos + W] == t);
        let ans = if ok { "Yes" } else { "No" };
        println!("{}", ans);
    } else {
        println!("No");
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
