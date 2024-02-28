#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

enum Event {
    Alphabet { ch: char, count: u64 },
    Numeric { base: u64 },
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        S: Chars,
        X: u64,
    };

    let mut count = 0_u64;
    let mut stack = vec![];
    for ch in S {
        if count >= X {
            break;
        }
        if ch.is_ascii_alphabetic() {
            count += 1;
            stack.push(Event::Alphabet { ch, count });
        } else {
            let n = ch.to_digit(10).unwrap() as u64;
            let base = count;
            count *= n + 1;
            stack.push(Event::Numeric { base });
        }
    }

    let mut rest = X;
    while let Some(event) = stack.pop() {
        match event {
            Event::Alphabet { ch, count } => {
                if count == rest {
                    println!("{}", ch);
                    return;
                }
            }
            Event::Numeric { base } => {
                if base > rest {
                    continue;
                }
                rest %= base;
                if rest == 0 {
                    rest = base;
                }
            }
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
