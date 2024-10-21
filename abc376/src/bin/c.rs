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
        mut A: [isize; N],
        mut B: [isize; N - 1],
    };

    A.sort_by_key(|&x| std::cmp::Reverse(x));
    B.sort_by_key(|&x| std::cmp::Reverse(x));
    B.push(-1_isize);
    let mut b_i = 0;
    let mut ans = None;
    for a in A {
        if B[b_i] >= a {
            b_i += 1;
        } else if ans.is_some() {
            // すでに箱を購入している場合は条件を達成できない
            println!("-1");
            return;
        } else {
            // 箱を購入する
            ans = Some(a);
        }
    }
    println!("{}", ans.unwrap());
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
