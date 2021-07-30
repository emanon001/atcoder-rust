#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn shortest_path(grid: Vec<Vec<char>>, start: (usize, usize), goal: (usize, usize), h: usize, w: usize, inf: i64) -> i64 {
    let mut cost_list = vec![vec![vec![inf; 4]; w]; h];
    let mut heap = std::collections::BinaryHeap::new();
    cost_list[start.0][start.1][0] = 0_i64;
    cost_list[start.0][start.1][1] = 0_i64;
    cost_list[start.0][start.1][2] = 0_i64;
    cost_list[start.0][start.1][3] = 0_i64;
    // (cost, dir, pos)
    heap.push(std::cmp::Reverse((0, 0, start)));
    heap.push(std::cmp::Reverse((0, 1, start)));
    heap.push(std::cmp::Reverse((0, 2, start)));
    heap.push(std::cmp::Reverse((0, 3, start)));
    while let Some(std::cmp::Reverse((cost, dir, pos))) = heap.pop() {
        if cost > cost_list[pos.0][pos.1][dir] {
            continue;
        }
        let directions = vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ];
        for k in 0..4 {
            let new_i = pos.0 as isize + directions[k].0;
            let new_j = pos.1 as isize + directions[k].1;
            if new_i < 0 || new_i == h as isize || new_j < 0 || new_j == w as isize {
                continue;
            }
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            if grid[new_i][new_j] == '#' {
                continue;
            }
            let new_cost = if dir == k { cost } else { cost + 1 };
            if new_cost < cost_list[new_i][new_j][k] {
                cost_list[new_i][new_j][k] = new_cost;
                heap.push(std::cmp::Reverse((new_cost, k, (new_i, new_j))));
            }
        }
    }
    *cost_list[goal.0][goal.1].iter().min().unwrap()
}

fn solve() {
    input! {
        h: usize, w: usize,
        rs: Usize1, cs: Usize1,
        rt: Usize1, ct: Usize1,
        grid: [Chars; h]
    };

    let res = shortest_path(grid, (rs, cs), (rt, ct), h, w, 1_i64 << 60);
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
