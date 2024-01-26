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
        H: usize, W: usize,
        grid: [Chars; H]
    };

    let (s_pos, a_pos) = {
        let mut s_pos = (0, 0);
        let mut a_pos = (0, 0);
        for i in 0..H {
            for j in 0..W {
                if grid[i][j] == 's' {
                    s_pos = (i, j);
                }
                if grid[i][j] == 'a' {
                    a_pos = (i, j);
                }
            }
        }
        (s_pos, a_pos)
    };
    let ans = traverse(s_pos, a_pos, H, W, &grid);
    println!("{}", ans);
}

fn traverse(
    s_pos: (usize, usize),
    a_pos: (usize, usize),
    h: usize,
    w: usize,
    grid: &[Vec<char>],
) -> isize {
    let mut dp = vec![vec![std::usize::MAX; h * w]; h * w];
    let mut que = VecDeque::new();
    que.push_back((s_pos, a_pos, 0));
    while let Some((s_pos, a_pos, step)) = que.pop_front() {
        let (s_i, s_j) = s_pos;
        let s_id = s_i * w + s_j;
        let (a_i, a_j) = a_pos;
        let a_id = a_i * w + a_j;
        if dp[s_id][a_id] <= step {
            continue;
        }
        if grid[a_i][a_j] == 'g' {
            return step as isize;
        }
        dp[s_id][a_id] = step;
        // 上下左右に移動する
        for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_s_i = s_i as isize + di;
            let new_s_j = s_j as isize + dj;
            if new_s_i < 0 || new_s_i >= h as isize || new_s_j < 0 || new_s_j >= w as isize {
                continue;
            }
            let new_s_i = new_s_i as usize;
            let new_s_j = new_s_j as usize;
            if grid[new_s_i][new_s_j] == '#' {
                continue;
            }
            let new_s_pos = (new_s_i, new_s_j);
            let new_a_pos = if new_s_pos == a_pos {
                // 荷物を移動できるか判定する
                let new_a_i = a_i as isize + di;
                let new_a_j = a_j as isize + dj;
                if new_a_i < 0 || new_a_i >= h as isize || new_a_j < 0 || new_a_j >= w as isize {
                    continue;
                }
                let new_a_i = new_a_i as usize;
                let new_a_j = new_a_j as usize;
                if grid[new_a_i][new_a_j] == '#' {
                    continue;
                }
                (new_a_i, new_a_j)
            } else {
                a_pos
            };
            que.push_back((new_s_pos, new_a_pos, step + 1));
        }
    }
    -1
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
