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
        N: usize, S: u128,
        A: [u128; N],
    };

    for a_perm in A.into_iter().permutations(N) {
        for bits in 0..(1 << (N - 1)) {
            let mut s = Vec::new();
            for i in 0..N - 1 {
                let op = if (bits >> i) & 1 == 0 { "+" } else { "x" }.to_owned();
                s.push(op);
                s.push(a_perm[i + 1].to_string());
            }
            if eval(a_perm[0] as u128, &s) == S {
                println!("Yes");
                println!("{}{}", a_perm[0], s.iter().join(""));
                return;
            }
        }
    }
    println!("No");
}

fn eval(l: u128, rest: &[String]) -> u128 {
    // eprintln!("l: {}, rest: {:?}", l, rest);
    if rest.is_empty() {
        return l;
    }
    match rest[0].as_str() {
        "x" => {
            let r = rest[1].parse::<u128>().unwrap();
            eval(l * r, &rest[2..])
        }
        "+" => {
            let r = rest[1].parse::<u128>().unwrap();
            l + eval(r, &rest[2..])
        }
        _ => unreachable!(),
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
