#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        S: Chars,
    };

    let mut one_pos = Vec::new();
    for i in 0..N {
        if S[i] == '1' {
            one_pos.push(i);
        }
    }
    let one_count = one_pos.len();
    let mid_pos = if one_count.is_odd() {
        vec![one_pos[one_count / 2]]
    } else {
        vec![one_pos[one_count / 2 - 1], one_pos[one_count / 2]]
    };

    let mut ans = 1 << 60;
    for mid in mid_pos {
        let mut cost = 0;
        let mut r = mid;
        for &p in one_pos.iter().rev() {
            if p >= r {
                continue;
            }
            cost += r - p - 1;
            r -= 1;
        }

        let mut l = mid;
        for &p in &one_pos {
            if p <= l {
                continue;
            }
            cost += p - l - 1;
            l += 1;
        }
        chmin!(ans, cost);
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
