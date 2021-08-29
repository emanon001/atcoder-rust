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
        n: usize, m: usize,
    };

    let mut kv = vec![vec![]; m];
    for i in 0..m {
        input! {
            k: usize,
            v: [i64; k]
        };
        kv[i] = v;
    }
    let mut ball_set = HashSet::new();
    let mut v_to_i = vec![n; n + 1];
    let mut pos = vec![0; m];
    let mut next = (0..m).collect::<Vec<_>>();
    let mut pop_count = 0;
    while pop_count < 2 * n {
        let mut new_next = vec![];
        let mut poped = false;
        for &i in &next {
            let v = kv[i][pos[i]];
            if ball_set.contains(&v) {
                poped = true;
                pos[i] += 1;
                if pos[i] < kv[i].len() {
                    new_next.push(i);
                }
                let j = v_to_i[v as usize];
                pos[j] += 1;
                if pos[j] < kv[j].len() {
                    new_next.push(j);
                }
                pop_count += 2;
            } else {
                let v = kv[i][pos[i]];
                v_to_i[v as usize] = i;
                ball_set.insert(v);
            }
        }
        if !poped {
            println!("No");
            return;
        }
        next = new_next;
    }
    println!("Yes");
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
