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

    let n = S.len();
    let counts = S.into_iter().counts();
    let mut ans = 0_u64;
    let mut is_duplicate = false;
    for (_, c) in counts {
        is_duplicate |= c > 1;
        ans += c as u64 * (n - c) as u64;
    }
    ans /= 2;
    ans = if is_duplicate { ans + 1 } else { ans };
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
