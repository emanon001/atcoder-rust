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

#[derive(Clone)]
struct Board {
    board: Vec<Vec<usize>>,
    n: usize,
    empty_tile_pos: (usize, usize),
}

impl Board {
    const OPERATIONS: [char; 4] = ['U', 'D', 'L', 'R'];
    fn new(board: Vec<Vec<usize>>, n: usize) -> Self {
        let mut empty_tile_pos = (0, 0);
        'outer: for i in 0..n {
            for j in 0..n {
                if board[i][j] == 0 {
                    empty_tile_pos = (i, j);
                    break 'outer;
                }
            }
        }
        Self {
            board,
            n,
            empty_tile_pos,
        }
    }

    fn from_operations(initial_board: &Board, operations: &Vec<char>) -> Self {
        let mut new_board = initial_board.clone();
        for op in operations {
            new_board.move_tile(*op);
        }
        new_board
    }

    fn move_tile(&mut self, op: char) {
        let (ei, ej) = self.empty_tile_pos;
        let (new_i, new_j) = match op {
            'U' => (ei - 1, ej),
            'D' => (ei + 1, ej),
            'L' => (ei, ej - 1),
            'R' => (ei, ej + 1),
            _ => unreachable!(),
        };
        self.board[ei][ej] = self.board[new_i][new_j];
        self.board[new_i][new_j] = 0;
        self.empty_tile_pos = (new_i, new_j);
    }

    fn rev_op(op: char) -> char {
        match op {
            'U' => 'D',
            'D' => 'U',
            'L' => 'R',
            'R' => 'L',
            _ => unreachable!(),
        }
    }

    fn is_rev_op(prev_op: char, op: char) -> bool {
        prev_op == Self::rev_op(op)
    }

    fn can_move_tile(&self, op: char) -> bool {
        let (ei, ej) = self.empty_tile_pos;
        let invalid = op == 'U' && ei == 0
            || op == 'D' && ei == self.n - 1
            || op == 'L' && ej == 0
            || op == 'R' && ej == self.n - 1;
        !invalid
    }

    fn get_tile(&self, i: usize, j: usize) -> usize {
        self.board[i][j]
    }
}

struct Scores {
    operation_map: HashMap<String, Vec<char>>,
    score_map: HashMap<String, f64>,
}

impl Scores {
    fn new(initial_score: f64) -> Self {
        let mut operation_map: HashMap<String, Vec<char>> = HashMap::new();
        operation_map.insert("".into(), vec![]);
        let mut score_map: HashMap<String, f64> = HashMap::new();
        score_map.insert("".into(), initial_score);
        Self {
            operation_map,
            score_map,
        }
    }

    fn update_if_needed(&mut self, operations: &Vec<char>, score: f64) {
        let key = Self::get_key(&operations);
        let max_score = self.score_map.entry(key).or_insert(0.0);
        if &score > max_score {
            *max_score = score;
            let key = Self::get_key(&operations);
            self.operation_map.insert(key, operations.clone());
        }
    }

    fn get_key(operations: &Vec<char>) -> String {
        format!("{}{}{}", operations[0], operations[1], operations[2])
    }

    fn max_operations(&self) -> Vec<char> {
        let mut max_score = -1.0;
        let mut max_operations = &Vec::new();
        for (k, score) in &self.score_map {
            if score > &max_score {
                max_score = *score;
                max_operations = &self.operation_map[k];
            }
        }
        max_operations.clone()
    }

    fn operations(&self) -> Vec<Vec<char>> {
        let mut res = Vec::new();
        for v in self.operation_map.values() {
            res.push(v.clone());
        }
        res
    }
}

struct Solver {
    n: usize,
    t: usize,
    initial_board: Board,
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
        let initial_board = Board::new(board, n);
        let start_time = Instant::now();
        Solver {
            n,
            t,
            initial_board,
            rng: rand::thread_rng(),
            start_time,
        }
    }

    pub fn solve(&mut self) {
        let mut board = self.initial_board.clone();
        let initial_score = self.calc_score(&board, 0);
        let mut scores = Scores::new(initial_score);
        let mut operations: Vec<char> = vec![];
        let mut count: usize = 0;
        self.solve_dfs(&mut count, 0, &mut board, &mut operations, &mut scores, 8);
        'outer: loop {
            for ops in scores.operations() {
                let mut ops = ops;
                let mut board = Board::from_operations(&self.initial_board, &ops);
                let max_depth = (self.t - ops.len()).min(8);
                let can_continue =
                    self.solve_dfs(&mut count, 0, &mut board, &mut ops, &mut scores, max_depth);
                if !can_continue {
                    break 'outer;
                }
            }
        }

        println!("{}", scores.max_operations().iter().join(""));
    }

    pub fn solve_dfs(
        &mut self,
        c: &mut usize,
        depth: usize,
        board: &mut Board,
        operations: &mut Vec<char>,
        scores: &mut Scores,
        max_depth: usize,
    ) -> bool {
        if depth >= max_depth {
            return true;
        }
        for &op in &Board::OPERATIONS {
            // 時間をチェック
            if *c % 100 == 0 && !self.check_time_limit() {
                return false;
            }
            // 元の状態に戻らないようチェック
            if let Some(prev_op) = operations.last() {
                if Board::is_rev_op(*prev_op, op) {
                    continue;
                }
            }
            if !board.can_move_tile(op) {
                continue;
            }
            *c += 1;
            operations.push(op);
            board.move_tile(op);
            if operations.len() >= 3 {
                let new_score = self.calc_score(&board, operations.len());
                scores.update_if_needed(&operations, new_score);
            }
            self.solve_dfs(c, depth + 1, board, operations, scores, max_depth);
            operations.pop();
            board.move_tile(Board::rev_op(op));
        }
        true
    }

    fn check_time_limit(&self) -> bool {
        let now = Instant::now();
        let limit = self.start_time + Duration::from_millis(2950);
        now < limit
    }

    fn calc_score(&self, board: &Board, move_count: usize) -> f64 {
        let n = self.n;
        let mut tc = TreeChecker::new(n * n);
        for i in 0..n - 1 {
            for j in 0..n - 1 {
                // 下方向に連結可能か
                if (board.get_tile(i, j) & 8 != 0) && (board.get_tile(i + 1, j) & 2 != 0) {
                    tc.unite(i * n + j, (i + 1) * n + j);
                }
                // 右方向に連結可能か
                if (board.get_tile(i, j) & 4 != 0) && (board.get_tile(i, j + 1) & 1 != 0) {
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
