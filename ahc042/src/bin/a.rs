#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
use rand::prelude::*;
#[allow(unused_imports)]
use std::collections::*;
use std::time;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Piece {
    Oni,
    Fuku,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
        };
        write!(f, "{}", s)
    }
}

struct OutputItem {
    d: Direction,
    p: usize,
}

struct MoveCost(usize);

struct Output(Vec<OutputItem>);

struct Solver {
    n: usize,
    board: Vec<Vec<Option<Piece>>>,
    rng: ThreadRng,
    started_at: time::Instant,
}

impl Solver {
    fn new(n: usize, board: Vec<Vec<char>>) -> Self {
        let board = board
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| match c {
                        'x' => Some(Piece::Oni),
                        'o' => Some(Piece::Fuku),
                        '.' => None,
                        _ => unreachable!(),
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self {
            n,
            board,
            rng: rand::thread_rng(),
            started_at: time::Instant::now(),
        }
    }
    fn solve(&mut self) -> Output {
        let max_count = 4 * self.n.pow(2);
        let mut count = 0;
        let mut output = Vec::new();
        'outer: loop {
            if count == max_count {
                break;
            }
            for i in 0..self.n {
                for j in 0..self.n {
                    if time::Duration::from_millis(1900) < self.started_at.elapsed() {
                        break 'outer;
                    }

                    if self.board[i][j] != Some(Piece::Oni) {
                        continue;
                    }

                    let mut moves = self.simulate_drop_piece(i, j);
                    if moves.is_empty() {
                        let next_moves = self.simulate_next_drop_piece(i, j);
                        if next_moves.is_empty() {
                            continue;
                        }
                        if next_moves.len() + count > max_count {
                            continue;
                        }
                        count += next_moves.len();
                        let mut i = i;
                        let mut j = j;
                        for (d, _) in next_moves {
                            output.extend(self.move_board(i, j, d, 1));
                            match d {
                                Direction::Up => {
                                    i -= 1;
                                }
                                Direction::Down => {
                                    i += 1;
                                }
                                Direction::Left => {
                                    j -= 1;
                                }
                                Direction::Right => {
                                    j += 1;
                                }
                            }
                        }
                        continue;
                    }
                    moves.sort_by_key(|(_, cost)| cost.0);

                    let (d, cost) = moves.first().unwrap();
                    if count + cost.0 > max_count {
                        continue;
                    }
                    count += cost.0;
                    output.extend(self.move_board(i, j, *d, cost.0));
                    continue 'outer;
                }
            }
        }
        Output(output)
    }

    fn move_board(&mut self, i: usize, j: usize, d: Direction, step: usize) -> Vec<OutputItem> {
        let mut output = Vec::new();
        match d {
            Direction::Up => {
                for ni in 0..self.n {
                    // update board
                    self.board[ni][j] = if ni + step < self.n {
                        self.board[ni + step][j]
                    } else {
                        None
                    };
                }
                // add output
                for _ in 0..step {
                    output.push(OutputItem {
                        d: Direction::Up,
                        p: j,
                    });
                }
            }
            Direction::Down => {
                for ni in (0..self.n).rev() {
                    // update board
                    self.board[ni][j] = if ni >= step {
                        self.board[ni - step][j]
                    } else {
                        None
                    };
                }
                // add output
                for _ in 0..step {
                    output.push(OutputItem {
                        d: Direction::Down,
                        p: j,
                    });
                }
            }
            Direction::Left => {
                for nj in 0..self.n {
                    // update board
                    self.board[i][nj] = if nj + step < self.n {
                        self.board[i][nj + step]
                    } else {
                        None
                    };
                }
                // add output
                for _ in 0..step {
                    output.push(OutputItem {
                        d: Direction::Left,
                        p: i,
                    });
                }
            }
            Direction::Right => {
                for nj in (0..self.n).rev() {
                    // update board
                    self.board[i][nj] = if nj >= step {
                        self.board[i][nj - step]
                    } else {
                        None
                    };
                }
                // add output
                for _ in 0..step {
                    output.push(OutputItem {
                        d: Direction::Right,
                        p: i,
                    });
                }
            }
        }
        output
    }

    fn simulate_drop_piece(&self, i: usize, j: usize) -> Vec<(Direction, MoveCost)> {
        let mut res = Vec::new();
        if (0..i).all(|i| self.board[i][j] != Some(Piece::Fuku)) {
            res.push((Direction::Up, MoveCost(i + 1)));
        }
        if (i + 1..self.n).all(|i| self.board[i][j] != Some(Piece::Fuku)) {
            res.push((Direction::Down, MoveCost(self.n - i)));
        }
        if (0..j).all(|j| self.board[i][j] != Some(Piece::Fuku)) {
            res.push((Direction::Left, MoveCost(j + 1)));
        }
        if (j + 1..self.n).all(|j| self.board[i][j] != Some(Piece::Fuku)) {
            res.push((Direction::Right, MoveCost(self.n - j)));
        }
        res
    }

    fn simulate_next_drop_piece(&mut self, i: usize, j: usize) -> Vec<(Direction, usize)> {
        // 鬼を削除できる位置に移動させる
        let mut res = Vec::new();
        // let d = match self.rng.gen_range(0..4) {
        //     0 => Direction::Up,
        //     1 => Direction::Down,
        //     2 => Direction::Left,
        //     3 => Direction::Right,
        //     _ => unreachable!(),
        // };
        // match d {
        // Direction::Up => {
        if self.board[0][j] != Some(Piece::Fuku) {
            res.push((Direction::Up, j));
            return res;
        }
        // }
        // Direction::Down => {
        if self.board[self.n - 1][j] != Some(Piece::Fuku) {
            res.push((Direction::Down, j));
            return res;
        }
        // }
        // Direction::Left => {
        if self.board[i][0] != Some(Piece::Fuku) {
            res.push((Direction::Left, i));
            return res;
        }
        // }
        // Direction::Right => {
        if self.board[i][self.n - 1] != Some(Piece::Fuku) {
            res.push((Direction::Right, i));
            return res;
        }
        // }
        // }

        res
    }
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        C: [Chars; N],
    };

    let mut solver = Solver::new(N, C);
    let output = solver.solve();
    println!(
        "{}",
        output
            .0
            .iter()
            .map(|item| format!("{} {}", item.d, item.p))
            .join("\n")
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
