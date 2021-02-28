#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        k: usize,
        s: Chars,
        t: Chars
    };

    let s = s.into_iter().take(4).map(|ch| ch.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
    let t = t.into_iter().take(4).map(|ch| ch.to_digit(10).unwrap() as usize).collect::<Vec<_>>();

    let mut all_counts = vec![k; 10];
    for &x in &s {
        all_counts[x] -= 1;
    }
    for &x in &t {
        all_counts[x] -= 1;
    }

    let mut all_count = 0;
    let mut win_count = 0;
    for a in 1..=9 {
        if all_counts[a] == 0 {
            continue;
        }
        all_counts[a] -= 1;
        let mut tscore = 0_i64;
        let mut counts = vec![0_i64; 10];
        counts[a] += 1_i64;
        for &x in &s {
            counts[x] += 1;
        }
        for x in 1..=9 {
            tscore += (x as i64) * 10.pow(counts[x] as u32);
        }
        for b in 1..=9 {
            if all_counts[b] == 0 {
                continue;
            }
            all_count += (all_counts[a] + 1) as i64 * all_counts[b] as i64;
            let mut ascore = 0_i64;
            let mut counts = vec![0_i64; 10];
            counts[b] += 1_i64;
            for &x in &t {
                counts[x] += 1;
            }
            for x in 1..=9 {
                ascore += (x as i64) * 10.pow(counts[x] as u32);
            }
            if tscore > ascore {
                win_count += (all_counts[a] + 1) as i64 * all_counts[b] as i64;
            }
        }
        all_counts[a] += 1;
    }
    let res = win_count as f64 / all_count as f64;
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
