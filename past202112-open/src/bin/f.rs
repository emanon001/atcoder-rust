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
        a: Usize1, b: Usize1,
        s: [Chars; 3],
    };

    let mut visited = vec![vec![false; 9]; 9];
    let mut que = VecDeque::new();
    que.push_back((a, b));
    visited[a][b] = true;
    while let Some((x, y)) = que.pop_front() {
        for i in 0..3 {
            for j in 0..3 {
                if s[i][j] == '.' {
                    continue;
                }

                let new_x = x as isize + i as isize + 1 - 2;
                let new_y = y as isize + j as isize + 1 - 2;
                if new_x < 0 || new_x >= 9 {
                    continue;
                }
                if new_y < 0 || new_y >= 9 {
                    continue;
                }
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if visited[new_x][new_y] {
                    continue;
                }
                visited[new_x][new_y] = true;
                que.push_back((new_x, new_y));
            }
        }
    }
    let mut res = 0;
    for i in 0..9 {
        for j in 0..9 {
            if visited[i][j] {
                res += 1;
            }
        }
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
