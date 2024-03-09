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
        A: [u64; M],
        S: [Chars; N],
    };

    let mut points = vec![0_u64; N];
    for i in 0..N {
        let mut point = i as u64 + 1;
        for j in 0..M {
            if S[i][j] == 'o' {
                point += A[j];
            }
        }
        points[i] = point;
    }
    let sorted_a = A
        .into_iter()
        .enumerate()
        .sorted_by_key(|(_, a)| std::cmp::Reverse(*a))
        .collect::<Vec<_>>();
    for (i, s) in S.into_iter().enumerate() {
        let max_point = *points
            .iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .max_by_key(|(_, p)| *p)
            .unwrap()
            .1;
        let mut cur_point = points[i];
        let mut ans = 0;
        for &(j, p) in &sorted_a {
            if cur_point > max_point {
                break;
            }
            if s[j] == 'x' {
                cur_point += p;
                ans += 1;
            }
        }
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
