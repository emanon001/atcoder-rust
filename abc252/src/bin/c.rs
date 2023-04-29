#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
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

fn solve() {
    input! {
        n: usize,
        sv: [Chars; n]
    };

    let sv = sv
        .into_iter()
        .map(|s| {
            s.into_iter()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = 1 << 30;
    for x in 0_u32..10 {
        let mut indexes = sv
            .iter()
            .map(|s| s.iter().position(|y| &x == y).unwrap() as isize)
            .collect::<Vec<_>>();
        let mut sorted = Vec::new();
        let mut cur = -1_isize;
        while !indexes.is_empty() {
            indexes.sort_by_key(|i| {
                if &cur < i {
                    i - cur
                } else {
                    n as isize - cur + i
                }
            });
            cur = indexes[0];
            sorted.push(cur);
            indexes.remove(0);
        }

        let mut sum = sorted[0];
        let mut pos = sorted[0];
        for &pos2 in &sorted[1..] {
            if pos < pos2 {
                sum += pos2 - pos;
            } else {
                sum += 10 - pos + pos2;
            }
            pos = pos2;
        }
        chmin!(res, sum);
    }
    println!("{}", res);
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
