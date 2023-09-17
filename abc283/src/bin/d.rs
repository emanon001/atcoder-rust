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

    let mut stack = Vec::new();
    let mut set = HashSet::new();
    let mut cur_set = HashSet::new();
    for ch in s {
        match ch {
            '(' => {
                stack.push(cur_set);
                cur_set = HashSet::new();
            }
            ')' => {
                for ch in cur_set {
                    set.remove(&ch);
                }
                cur_set = stack.pop().unwrap_or(HashSet::new());
            }
            ch => {
                if set.contains(&ch) {
                    println!("No");
                    return;
                }
                set.insert(ch);
                cur_set.insert(ch);
            }
        }
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
