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
        S: [Chars; H],
    };

    let mut s = (0, 0);
    let mut g = (0, 0);
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == 'S' {
                s = (i, j);
            }
            if S[i][j] == 'G' {
                g = (i, j);
            }
        }
    }

    let ans = shortest_path(s, &S)[g.0 * W + g.1].unwrap_or(-1);
    println!("{}", ans);
}

const INF: i64 = 1_i64 << 60;

#[allow(non_snake_case)]
fn shortest_path(start: (usize, usize), grid: &[Vec<char>]) -> Vec<Option<i64>> {
    let (H, W) = (grid.len(), grid[0].len());
    let mut cost_list = vec![vec![INF; 2]; H * W];
    let mut heap = BinaryHeap::new();
    let u = start.0 * W + start.1;
    cost_list[u][0] = 0;
    cost_list[u][1] = 0;
    heap.push((std::cmp::Reverse(0), u, 0));
    heap.push((std::cmp::Reverse(0), u, 1));
    while let Some((std::cmp::Reverse(cost), u, d)) = heap.pop() {
        if cost > cost_list[u][d] {
            continue;
        }
        for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            // d: 0 -> 横移動, 1 -> 縦移動
            if (d == 0 && di == 0) || (d == 1 && dj == 0) {
                continue;
            }
            let (i, j) = (u / W, u % W);
            let (ni, nj) = (i as isize + di, j as isize + dj);
            if ni < 0 || ni >= H as isize || nj < 0 || nj >= W as isize {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if grid[ni][nj] == '#' {
                continue;
            }
            let new_u = ni * W + nj;
            let new_cost = cost + 1;
            let new_d = (d + 1) % 2;
            if new_cost < cost_list[new_u][new_d] {
                cost_list[new_u][new_d] = new_cost;
                heap.push((std::cmp::Reverse(new_cost), new_u, new_d));
            }
        }
    }
    cost_list
        .into_iter()
        .map(|v| {
            let x = v[0].min(v[1]);
            if x == INF {
                None
            } else {
                Some(x)
            }
        })
        .collect()
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
