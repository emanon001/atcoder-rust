#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub struct Grid {
    grid: Vec<Vec<char>>,
    h: usize,
    w: usize,
    ng_char: char,
}
impl Grid {
    pub fn new(grid: &[Vec<char>], ng_char: char) -> Self {
        assert!(grid.len() > 0);
        let grid = grid.into_iter().cloned().collect::<Vec<_>>();
        let h = grid.len();
        let w = grid[0].len();
        Self {
            grid,
            h,
            w,
            ng_char,
        }
    }
    pub fn shortest_path(&self, start: (usize, usize), goal: (usize, usize)) -> i64 {
        let mut cost_table = HashMap::new();
        let mut que = std::collections::VecDeque::new();
        cost_table.insert(start, 0_i64);
        que.push_front((start, 0_i64));
        while let Some((from, cost)) = que.pop_front() {
            if &cost > cost_table.get(&from).unwrap() {
                continue;
            }
            let (i, j) = from;
            for &(di, dj) in &UDLR_DIRS {
                let new_i = i as isize + di;
                let new_j = j as isize + dj;
                if !self.in_grid(new_i, new_j) {
                    continue;
                }
                let new_i = new_i as usize;
                let new_j = new_j as usize;
                if (new_i, new_j) == goal {
                    return cost;
                }
                let w = if self.cell(new_i, new_j) == self.ng_char() {
                    1
                } else {
                    0
                };
                let new_cost = cost + w;
                let to = (new_i, new_j);
                if !cost_table.contains_key(&to) || &new_cost < cost_table.get(&to).unwrap() {
                    cost_table.insert(to, new_cost);
                    if w == 0 {
                        que.push_front((to, new_cost));
                    } else {
                        que.push_back((to, new_cost));
                    }
                }
            }
        }
        unreachable!();
    }
    pub fn height(&self) -> usize {
        self.h
    }
    pub fn width(&self) -> usize {
        self.w
    }
    pub fn in_grid(&self, i: isize, j: isize) -> bool {
        i >= 0 && i < self.h as isize && j >= 0 && j < self.w as isize
    }
    pub fn cell(&self, i: usize, j: usize) -> char {
        self.grid[i][j]
    }
    pub fn ng_char(&self) -> char {
        self.ng_char
    }
}
/// 上下左右 (i, j)
pub const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn solve() {
    input! {
        h: usize, w: usize,
        grid: [Chars; h]
    };

    let mut s = (0, 0);
    let mut g = (0, 0);
    for i in 0..h {
        for j in 0..w {
            match grid[i][j] {
                's' => s = (i, j),
                'g' => g = (i, j),
                _ => {}
            };
        }
    }

    let grid = Grid::new(&grid, '#');
    let cost = grid.shortest_path(s, g);
    let res = if cost <= 2 { "YES" } else { "NO" };
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
