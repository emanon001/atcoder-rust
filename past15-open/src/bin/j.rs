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
        N: usize, M: u64,
        mut ABX: [(u64, u64, usize); N]
    };

    // (ソート用キー, 一度だけ手裏剣を投げるか, index)
    let mut actions = vec![];
    for (i, (a, b, x)) in ABX.iter().copied().enumerate() {
        match (x == 0, b == 1) {
            (true, true) => {
                // 倒せるので優先度を下げる
                actions.push((std::cmp::Reverse(0), false, i));
            }
            (true, false) => {
                actions.push((std::cmp::Reverse(a), false, i));
            }
            (false, true) => {
                actions.push((std::cmp::Reverse(a), false, i));
            }
            (false, false) => {
                actions.push((std::cmp::Reverse(a * 2), true, i));
                actions.push((std::cmp::Reverse(a), false, i));
            }
        }
    }

    actions.sort();
    let mut ans = 0_u64;
    let mut rest_m = M;
    for (_, once_throw, i) in actions {
        let (a, b, x) = ABX[i];
        if b == 0 {
            continue;
        }
        if rest_m > 0 {
            // 手裏剣を投げられる
            if once_throw {
                ABX[i] = (a, b - 1, 0);
                rest_m -= 1;
            } else {
                let used_m = match (x == 0, b == 1) {
                    (true, true) => 0,
                    (true, false) => (b - 1).min(rest_m),
                    (false, true) => 1,
                    (false, false) => (b - 1).min(rest_m),
                };
                rest_m -= used_m;
                ans += a * if b - used_m == 0 { 0 } else { b - used_m - 1 };
                ABX[i] = (a, 0, 0);
            }
        } else {
            let b = if x == 0 { b - 1 } else { b };
            ans += a * b;
            ABX[i] = (a, 0, x);
        }
        // eprintln!(
        //     "(a: {}, b: {}, x: {}), ans: {}, rest_m: {}",
        //     a, b, x, ans, rest_m
        // );
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
