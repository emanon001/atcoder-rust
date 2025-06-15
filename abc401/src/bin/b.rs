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
        N: usize,
        S: [String; N]
    };

    let mut ans = 0;
    let mut logged_in = false;
    for s in S {
        match s.as_str() {
            "login" => logged_in = true,
            "logout" => logged_in = false,
            "private" => {
                if !logged_in {
                    ans += 1;
                }
            }
            _ => {}
        }
    }
    println!("{}", ans);
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
