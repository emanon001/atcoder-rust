#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use rand::prelude::*;
#[allow(unused_imports)]
use std::collections::*;
use std::time::{Duration, Instant};

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

// base: union-find
pub struct TreeChecker {
    n: usize,
    root: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
    is_tree: Vec<bool>,
}
impl TreeChecker {
    pub fn new(n: usize) -> Self {
        let root = (0..n).collect();
        let rank = vec![0; n];
        let size = vec![1; n];
        let is_tree = vec![true; n];
        Self {
            n,
            root,
            rank,
            size,
            is_tree,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        if self.root[x] == x {
            x
        } else {
            let root = self.find(self.root[x]);
            self.root[x] = root;
            root
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        assert!(x < self.n && y < self.n);
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            self.is_tree[x_root] = false;
            self.is_tree[y_root] = false;
            return;
        }
        if self.rank[x_root] < self.rank[y_root] {
            self.root[x_root] = y_root;
            self.size[y_root] += self.size[x_root];
        } else {
            self.root[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }
    pub fn size(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        let x_root = self.find(x);
        self.size[x_root]
    }
    pub fn is_tree(&mut self, x: usize) -> bool {
        let x_root = self.find(x);
        self.is_tree[x_root]
    }
    pub fn max_size(&mut self) -> usize {
        let mut max_size = 0;
        for x in 0..self.n {
            if self.is_tree(x) {
                let size = self.size(x);
                chmax!(max_size, size);
            }
        }
        max_size
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.n && y < self.n);
        self.find(x) == self.find(y)
    }
}

struct Solver {
    n: usize,
    t: usize,
    board: Vec<Vec<usize>>,
    empty_tile_pos: (usize, usize),
    rng: ThreadRng,
    start_time: Instant,
}

impl Solver {
    fn new(n: usize, t: usize, board: Vec<Vec<char>>) -> Self {
        let board = board
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|ch| usize::from_str_radix(&ch.to_string(), 16).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut empty_tile_pos = (0, 0);
        'outer: for i in 0..n {
            for j in 0..n {
                if board[i][j] == 0 {
                    empty_tile_pos = (i, j);
                    break 'outer;
                }
            }
        }
        let start_time = Instant::now();
        Solver {
            n,
            t,
            board,
            empty_tile_pos,
            rng: rand::thread_rng(),
            start_time,
        }
    }

    pub fn solve(&mut self) {
        let mut max_score = self.calc_score(&self.board, 0);
        let mut max_operations: Vec<char> = vec![];
        let mut operations: Vec<char> = vec![];
        let mut count: usize = 0;
        self.solve_dfs(
            &mut count,
            0,
            self.empty_tile_pos,
            &mut operations,
            &mut max_operations,
            &mut max_score,
            12,
        );
        let depth = operations.len();
        self.solve_dfs(
            &mut count,
            0,
            self.empty_tile_pos,
            &mut operations,
            &mut max_operations,
            &mut max_score,
            self.t - depth,
        );
        println!("{}", max_operations.iter().join(""));
    }

    pub fn solve_dfs(
        &mut self,
        c: &mut usize,
        depth: usize,
        empty_tile_pos: (usize, usize),
        operations: &mut Vec<char>,
        max_operations: &mut Vec<char>,
        max_score: &mut f64,
        max_depth: usize,
    ) {
        if depth >= max_depth {
            return;
        }
        let dirs = vec!['U', 'D', 'L', 'R'];
        for &d in &dirs {
            // 時間をチェック
            if *c % 50 == 0 && !self.check_time_limit() {
                break;
            }
            // 元の状態に戻らないようチェック
            if let Some(last_op) = operations.last() {
                if last_op == &'D' && d == 'U'
                    || last_op == &'U' && d == 'D'
                    || last_op == &'R' && d == 'L'
                    || last_op == &'L' && d == 'R'
                {
                    continue;
                }
            }
            let (ei, ej) = empty_tile_pos;
            if d == 'U' && ei == 0
                || d == 'D' && ei == self.n - 1
                || d == 'L' && ej == 0
                || d == 'R' && ej == self.n - 1
            {
                continue;
            }
            *c += 1;
            operations.push(d);
            let (new_i, new_j) = match d {
                'U' => (ei - 1, ej),
                'D' => (ei + 1, ej),
                'L' => (ei, ej - 1),
                'R' => (ei, ej + 1),
                _ => unreachable!(),
            };
            self.board[ei][ej] = self.board[new_i][new_j];
            self.board[new_i][new_j] = 0;
            self.empty_tile_pos = (new_i, new_j);
            let new_score = self.calc_score(&self.board, operations.len());
            if &new_score > max_score {
                *max_score = new_score;
                *max_operations = operations.clone();
            }
            self.solve_dfs(
                c,
                depth + 1,
                (new_i, new_j),
                operations,
                max_operations,
                max_score,
                max_depth,
            );
            operations.pop();
            self.board[new_i][new_j] = self.board[ei][ej];
            self.board[ei][ej] = 0;
            self.empty_tile_pos = (ei, ej);
        }
    }

    fn check_time_limit(&self) -> bool {
        let now = Instant::now();
        let limit = self.start_time + Duration::from_millis(2950);
        now < limit
    }

    fn calc_score(&self, board: &Vec<Vec<usize>>, move_count: usize) -> f64 {
        let n = self.n;
        let mut tc = TreeChecker::new(n * n);
        for i in 0..n - 1 {
            for j in 0..n - 1 {
                // 下方向に連結可能か
                if (board[i][j] & 8 != 0) && (board[i + 1][j] & 2 != 0) {
                    tc.unite(i * n + j, (i + 1) * n + j);
                }
                // 右方向に連結可能か
                if (board[i][j] & 4 != 0) && (board[i][j + 1] & 1 != 0) {
                    tc.unite(i * n + j, i * n + j + 1);
                }
            }
        }
        let max_size = tc.max_size();
        if max_size < n * n - 1 {
            500000.0 as f64 * (max_size as f64 / (n as f64 * n as f64 - 1.0))
        } else {
            500000.0 as f64 * (max_size as f64 / (2.0 - (move_count as f64 / self.t as f64)))
        }
    }
}

fn main() {
    input! {
        n: usize, t: usize,
        board: [Chars; n],
    };
    let mut solver = Solver::new(n, t, board);
    solver.solve();
}
