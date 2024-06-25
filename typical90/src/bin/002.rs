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
    };

    let mut ans = vec![];
    for bits in 0..1 << N {
        let mut v = vec![];
        let mut c = 0_isize;
        for i in 0..N {
            if bits & 1 << i == 0 {
                c += 1;
                v.push('(');
            } else {
                c -= 1;
                v.push(')');
            }
            if c < 0 {
                break;
            }
        }
        if c != 0 {
            continue;
        }
        ans.push(v);
    }
    ans.sort();
    println!("{}", ans.iter().map(|v| v.iter().join("")).join("\n"));
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
