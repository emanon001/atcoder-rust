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
        H: usize, W: usize,
        A: [[usize; W]; H]
    };

    let mut ans = BTreeSet::new();
    for i in 0..H {
        for j in 0..W {
            for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni < 0 || ni >= H as isize || nj < 0 || nj >= W as isize {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                let a = A[i][j];
                let b = A[ni][nj];
                ans.insert((a.min(b), a.max(b)));
            }
        }
    }
    println!(
        "{}",
        ans.iter().map(|(a, b)| format!("{} {}", a, b)).join("\n")
    );
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
