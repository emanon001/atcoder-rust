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
        S: [Chars; 8],
    };

    let mut v_set = HashSet::new();
    let mut h_set = HashSet::new();
    for i in 0..8 {
        for j in 0..8 {
            if S[i][j] == '#' {
                v_set.insert(i);
                h_set.insert(j);
            }
        }
    }

    let mut ans = 0;
    for i in 0..8 {
        for j in 0..8 {
            if S[i][j] == '#' {
                continue;
            }
            if !v_set.contains(&i) && !h_set.contains(&j) {
                ans += 1;
            }
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
