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
        N: usize, M: usize,
        mut A: [usize; N],
        mut B: [usize; M],
    };
    A.sort();
    B.sort();
    let mut ans = 0;
    let mut i = 0;
    for j in 0..M {
        while i < N && A[i] < B[j] {
            i += 1;
        }
        if i == N {
            ans = 0;
            break;
        }
        ans += A[i];
        i += 1;
    }

    if ans == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
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
