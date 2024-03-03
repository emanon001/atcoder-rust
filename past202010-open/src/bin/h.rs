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
        N: usize, M: usize, K: usize,
        S: [Chars; N],
    };

    for n in (1..=N).rev() {
        for i in 0..N {
            if i + n > N {
                break;
            }
            for j in 0..M {
                if j + n > M {
                    break;
                }
                let mut counts = vec![0; 10];
                for di in 0..n {
                    for dj in 0..n {
                        let v = S[i + di][j + dj].to_digit(10).unwrap() as usize;
                        counts[v] += 1;
                    }
                }
                let max_count = counts.iter().max().unwrap();
                if max_count + K >= n * n {
                    println!("{}", n);
                    return;
                }
            }
        }
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
