#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use ordered_float::NotNan;
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
            return;
        }
        let is_tree = self.is_tree[x_root] && self.is_tree[y_root];
        if self.rank[x_root] < self.rank[y_root] {
            self.root[x_root] = y_root;
            self.size[y_root] += self.size[x_root];
            self.is_tree[y_root] = is_tree;
        } else {
            self.root[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
            self.is_tree[x_root] = is_tree;
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
    pub fn get_tree_sizes(&mut self) -> Vec<(usize, bool)> {
        let mut set = HashSet::new();
        for x in 0..self.n {
            let k = self.find(x);
            set.insert(k);
        }
        set.into_iter()
            .map(|x| (self.size(x), self.is_tree(x)))
            .collect()
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

    fn move_tile(&mut self, op: char) -> bool {
        let (ei, ej) = self.empty_tile_pos;
        let (new_i, new_j) = match op {
            'U' => (ei - 1, ej),
            'D' => (ei + 1, ej),
            'L' => (ei, ej - 1),
            'R' => (ei, ej + 1),
            _ => unreachable!(),
        };
        let connected = self.connected(new_i, new_j);
        self.board[ei][ej] = self.board[new_i][new_j];
        self.board[new_i][new_j] = 0;
        self.empty_tile_pos = (new_i, new_j);
        connected || self.connected(ei, ej) || self.connected(new_i, new_j)
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

    fn connected(&self, i: usize, j: usize) -> bool {
        self.connected_top(i, j)
            || self.connected_bottom(i, j)
            || self.connected_left(i, j)
            || self.connected_right(i, j)
    }

    fn connected_top(&self, i: usize, j: usize) -> bool {
        i > 0 && (self.board[i][j] & 2 != 0) && (self.board[i - 1][j] & 8 != 0)
    }

    fn connected_bottom(&self, i: usize, j: usize) -> bool {
        i < self.n - 1 && (self.board[i][j] & 8 != 0) && (self.board[i + 1][j] & 2 != 0)
    }

    fn connected_left(&self, i: usize, j: usize) -> bool {
        j > 0 && (self.board[i][j] & 1 != 0) && (self.board[i][j - 1] & 4 != 0)
    }

    fn connected_right(&self, i: usize, j: usize) -> bool {
        j < self.n - 1 && (self.board[i][j] & 4 != 0) && (self.board[i][j + 1] & 1 != 0)
    }

    fn can_connect(&self, i: usize, j: usize, i2: usize, j2: usize) -> bool {
        ((self.board[i][j] & 2 != 0) && (self.board[i2][j2] & 8 != 0))
            || ((self.board[i][j] & 8 != 0) && (self.board[i2][j2] & 2 != 0))
            || ((self.board[i][j] & 1 != 0) && (self.board[i2][j2] & 4 != 0))
            || ((self.board[i][j] & 4 != 0) && (self.board[i2][j2] & 1 != 0))
    }

    fn get_tile(&self, i: usize, j: usize) -> usize {
        self.board[i][j]
    }
}

type Score = (NotNan<f64>, NotNan<f64>);
type Operations = Vec<char>;

trait ScoreStorage {
    fn update_if_needed(&mut self, operations: &Operations, score: Score);

    fn get_max_operations(&self) -> Operations;

    fn get_scores(&self) -> Vec<(Score, Operations)>;
}

struct OrderedScores {
    score_map: BTreeMap<Score, Vec<char>>,
    max_size: usize,
}

impl OrderedScores {
    fn new(max_size: usize) -> Self {
        let score_map: BTreeMap<Score, Vec<char>> = BTreeMap::new();
        Self {
            score_map,
            max_size,
        }
    }
    fn update_max_size(&mut self, max_size: usize) {
        assert!(max_size >= self.max_size);
        self.max_size = max_size;
    }
}

impl ScoreStorage for OrderedScores {
    fn update_if_needed(&mut self, operations: &Vec<char>, score: Score) {
        let mut remove_key: Option<Score> = None;
        if let Some((s, _)) = self.score_map.iter().next() {
            if self.score_map.len() >= self.max_size {
                if &score < s {
                    return;
                }
                remove_key = Some(s.clone());
            }
        }
        if let Some(key) = remove_key {
            self.score_map.remove(&key);
        }
        self.score_map.insert(score, operations.clone());
    }

    fn get_max_operations(&self) -> Vec<char> {
        self.score_map.iter().next_back().unwrap().1.clone()
    }

    fn get_scores(&self) -> Vec<(Score, Vec<char>)> {
        let mut res = Vec::new();
        for v in self.score_map.iter().rev() {
            res.push((v.0.clone(), v.1.clone()));
        }
        res
    }
}

struct ScoreCalculator {}

impl ScoreCalculator {
    pub fn calculate(
        board: &Board,
        move_count: usize,
        max_move_count: usize,
        needs_calc_tree: bool,
        prev_score: Option<&Score>,
    ) -> Score {
        let n = board.n;
        let score1 = if needs_calc_tree {
            let n = board.n;
            let mut tc = Self::create_tree_checker(board);
            let max_size = tc.max_size();
            if max_size < n * n - 1 {
                500000.0 as f64 * (max_size as f64 / (n as f64 * n as f64 - 1.0))
            } else {
                500000.0 as f64
                    * (max_size as f64 / (2.0 - (move_count as f64 / max_move_count as f64)))
            }
        } else {
            prev_score.unwrap().1.into_inner()
        };

        let mut score2 = 0.0;

        // 愚形を避ける
        for i in 0..n {
            for j in 0..n {
                let tile = board.get_tile(i, j);
                if ((tile & 1) != 0 && j == 0)
                    || ((tile & 2) != 0 && i == 0)
                    || ((tile & 4) != 0 && j == n - 1)
                    || ((tile & 8) != 0 && i == n - 1)
                {
                    score2 -= 10.0;
                }
                // ←
                // if (tile & 1) != 0 {
                // }
                // ↑
                // if (tile & 2) != 0 {
                // }
                // →
                // if (tile & 4) != 0 {
                // }
                // ↓
                // if (tile & 8) != 0 {
                // }

                // if !board.connected(i, j) {
                //     let dirs = vec![
                //         (-2, 0),
                //         (-1, 1),
                //         (0, 2),
                //         (1, 1),
                //         (2, 0),
                //         (-1, 1),
                //         (-2, 0),
                //         (-1, -1),
                //         (-2, 0),
                //     ];
                //     for d in &dirs {
                //         let i2 = i as isize + d.0;
                //         let j2 = j as isize + d.1;
                //         if i2 < 0 || i2 >= n as isize || j2 < 0 || j2 >= n as isize {
                //             continue;
                //         }
                //         let i2 = i2 as usize;
                //         let j2 = j2 as usize;
                //         if board.can_connect(i, j, i2, j2) {
                //             score2 += 1.0;
                //         }
                //     }
                // }
                // // ↑
                // if tile == 0x2 {
                //     if i > 0 && board.get_tile(i - 1, j) == 0x8 {
                //         score -= minus_score;
                //     }
                //     if i < n - 1 && tile == board.get_tile(i + 1, j) {
                //         score -= minus_score;
                //     }
                // }
                // // ←↑
                // if tile == 0x3 {
                //     if i < n - 1 && tile == board.get_tile(i + 1, j) {
                //         score -= minus_score;
                //     }
                //     if j < n - 1 && tile == board.get_tile(i, j + 1) {
                //         score -= minus_score;
                //     }
                // }
                // // →
                // if tile == 0x4 {
                //     if j < n - 1 && board.get_tile(i, j + 1) == 0x1 {
                //         score -= minus_score;
                //     }
                //     if j < n - 1 && tile == board.get_tile(i, j + 1) {
                //         score -= minus_score;
                //     }
                // }
                //// ←→
                // // if tile == 0x5 &&  {
                // //     score -= 10.0;
                // // }
                //// ↑→
                // if tile == 0x6 && i == 0 && j == n - 1 {
                //     score -= 10.0;
                // }
                //// ←↑→
                // // if tile == 0x7 {
                // //     score -= 10.0;
                // // }
                // // ↓
                // if tile == 0x8 {
                //     if i < n - 1 && board.get_tile(i + 1, j) == 0x2 {
                //         score -= minus_score;
                //     }
                //     if i < n - 1 && tile == board.get_tile(i + 1, j) {
                //         score -= minus_score;
                //     }
                // }
                //// ←↓
                // if tile == 0x9 {
                //     score -= 10.0;
                // }
                //// ↑↓
                // // if tile == 0xa {
                // //     score -= 10.0;
                // // }
                //// ←↑↓
                // // if tile == 0xb {
                // //     score -= 10.0;
                // // }
                //// ↓→
                // if tile == 0xc {
                //     score -= 10.0;
                // }
                // // ←→↓
                // // if tile == 0xd {
                // //     score -= 10.0;
                // // }
                // // ↑↓→
                // // if tile == 0xe {
                // //     score -= 10.0;
                // // }
                // ←↑→↓
                // if tile == 0xf {
                // }
            }
        }
        ((score1 + score2).into(), score1.into())
    }

    fn create_tree_checker(board: &Board) -> TreeChecker {
        let n = board.n;
        let mut tc = TreeChecker::new(n * n);
        for i in 0..n {
            for j in 0..n {
                // 下方向に連結可能か
                if board.connected_bottom(i, j) {
                    tc.unite(i * n + j, (i + 1) * n + j);
                }
                // 右方向に連結可能か
                if board.connected_right(i, j) {
                    tc.unite(i * n + j, i * n + j + 1);
                }
            }
        }
        tc
    }
}

struct Solver {
    n: usize,
    t: usize,
    initial_board: Board,
    start_time: Instant,
    rng: ThreadRng,
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
            start_time,
            rng: rand::thread_rng(),
        }
    }

    pub fn solve(&mut self) {
        let initial_board = self.initial_board.clone();

        // 初期盤面を登録
        let mut scores = OrderedScores::new(10);
        let initial_score = ScoreCalculator::calculate(&initial_board, 0, self.t, true, None);
        scores.update_if_needed(&vec![], initial_score);

        let mut max_score = (initial_score, vec![]);

        // スコアの高い盤面に更新する
        let mut loop_count = 0;
        let mut count: usize = 0;
        let depth = 7;
        loop {
            loop_count += 1;

            // 時間を確認
            let now = Instant::now();
            let limit = self.start_time + Duration::from_millis(2950);
            if now >= limit {
                break;
            }

            let tmp_scores = scores.get_scores();

            if loop_count == 2 {
                // ランダム盤面を作成
                scores = OrderedScores::new(20);
                for _ in 0..100 {
                    let (random_board, random_ops) = self.gen_random_board(
                        &initial_board,
                        ((loop_count - 1) * depth + 1)..(loop_count * depth + 1),
                    );
                    let random_score = ScoreCalculator::calculate(
                        &random_board,
                        random_ops.len(),
                        self.t,
                        true,
                        None,
                    );
                    scores.update_if_needed(&random_ops, random_score);
                }
                scores.update_max_size(30);
            } else {
                scores = OrderedScores::new(10);
            }

            for (s, ops) in tmp_scores {
                if s > max_score.0 {
                    max_score = (s, ops.clone());
                }

                let mut ops = ops;
                let mut board = Board::from_operations(&self.initial_board, &ops);
                let max_depth = (self.t - ops.len()).min(depth);
                self.solve_dfs(
                    &mut count,
                    0,
                    &mut board,
                    &mut ops,
                    &mut scores,
                    max_depth,
                    &s,
                    &limit,
                );
            }
        }

        // eprintln!("{}", loop_count);
        // eprintln!("{}", scores.get_max_score());
        println!("{}", max_score.1.iter().join(""));
    }

    pub fn solve_dfs<T: ScoreStorage>(
        &mut self,
        c: &mut usize,
        depth: usize,
        board: &mut Board,
        operations: &mut Vec<char>,
        scores: &mut T,
        max_depth: usize,
        prev_score: &Score,
        limit: &Instant,
    ) -> bool {
        if depth >= max_depth {
            return true;
        }
        for &op in &Board::OPERATIONS {
            // 時間をチェック
            if *c % 200 == 0 {
                let now = Instant::now();
                if &now >= limit {
                    return false;
                }
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
            let effected_tree = board.move_tile(op);
            let new_score = ScoreCalculator::calculate(
                &board,
                operations.len(),
                self.t,
                effected_tree,
                Some(&prev_score),
            );
            scores.update_if_needed(&operations, new_score);
            let can_continue = self.solve_dfs(
                c,
                depth + 1,
                board,
                operations,
                scores,
                max_depth,
                &new_score,
                limit,
            );
            operations.pop();
            board.move_tile(Board::rev_op(op));
            if !can_continue {
                return false;
            }
        }
        true
    }

    fn gen_random_board(
        &mut self,
        initial_board: &Board,
        move_count_range: std::ops::Range<usize>,
    ) -> (Board, Vec<char>) {
        let mut board = initial_board.clone();
        let mut ops = Vec::new();
        for _ in 0..self
            .rng
            .gen_range(move_count_range.start, move_count_range.end)
        {
            loop {
                let i = self.rng.gen::<usize>() % Board::OPERATIONS.len();
                let op = Board::OPERATIONS[i];
                if let Some(prev_op) = ops.last() {
                    if Board::is_rev_op(*prev_op, op) {
                        continue;
                    }
                }
                if !board.can_move_tile(op) {
                    continue;
                }
                ops.push(op);
                board.move_tile(op);
                break;
            }
        }
        (board, ops)
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
