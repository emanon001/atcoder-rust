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
    };

    let mut stack = Vec::new();
    for ch in S {
        match ch {
            '(' => {
                stack.push(ch);
            }
            ')' => match stack.pop() {
                Some('(') => {}
                _ => {
                    println!("No");
                    return;
                }
            },
            '[' => {
                stack.push(ch);
            }
            ']' => match stack.pop() {
                Some('[') => {}
                _ => {
                    println!("No");
                    return;
                }
            },
            '<' => {
                stack.push(ch);
            }
            '>' => match stack.pop() {
                Some('<') => {}
                _ => {
                    println!("No");
                    return;
                }
            },
            _ => unreachable!(),
        }
    }
    let ans = if stack.is_empty() { "Yes" } else { "No" };
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
