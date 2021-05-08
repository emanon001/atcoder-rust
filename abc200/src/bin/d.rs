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
        n: usize,
        av: [usize; n]
    };

    let mut dp1 = vec![vec![false; 200]; n + 1];
    let mut dp2: Vec<Vec<HashSet<Vec<usize>>>> = vec![vec![HashSet::new(); 200]; n + 1];
    dp1[0][0] = true;
    for i in 0..n {
        for j in 0..200 {
            dp1[i + 1][j] |= dp1[i][j];
            if !dp1[i][j] {
                continue;
            }
            for k in dp2[i][j].clone() {
                if dp2[i + 1][j].len() >= 2 {
                    break;
                }
                dp2[i + 1][j].insert(k);
            }

            // 到達可能
            let a = av[i];
            let new_j = (j + a) % 200;
            dp1[i + 1][new_j] = true;
            for mut k in dp2[i][j].clone() {
                if dp2[i + 1][new_j].len() >= 2 {
                    break;
                }
                k.push(i + 1);
                dp2[i + 1][new_j].insert(k);
            }
            if j == 0 {
                dp2[i + 1][new_j].insert(vec![i + 1]);
            }
        }
    }

    for i in 0..200 {
        let paths = dp2[n][i].clone();
        if paths.len() < 2 {
            continue;
        }
        println!("Yes");
        let mut c = 0;
        for p in paths {
            if c == 2 {
                break;
            }
            println!("{} {}", p.len(), p.iter().join(" "));
            c += 1;
        }
        return;
    }
    println!("No");
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
