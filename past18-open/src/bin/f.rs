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
        N: usize,
        variables: [(char, isize); N],
    };

    let variables = variables.into_iter().collect::<HashMap<_, _>>();
    let mut ans = eval(S[0], &variables);
    let mut i = 1;
    while i < S.len() {
        let op = S[i];
        let b = eval(S[i + 1], &variables);
        ans = match op {
            '+' => ans + b,
            '-' => ans - b,
            _ => unreachable!(),
        };
        i += 2;
    }
    println!("{}", ans);
}

fn eval(a: char, variables: &HashMap<char, isize>) -> isize {
    if a.is_numeric() {
        a.to_digit(10).unwrap() as isize
    } else {
        *variables.get(&a).unwrap()
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
