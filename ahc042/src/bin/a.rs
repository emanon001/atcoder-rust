#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
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
    started_at: time::Instant,
    board: Vec<Vec<Option<Piece>>>,
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
                        continue;
                    }
                    moves.sort_by_key(|(_, cost)| cost.0);

                    let (d, cost) = moves.first().unwrap();
                    if count + cost.0 > max_count {
                        continue;
                    }
                    count += cost.0;
                    match d {
                        Direction::Up => {
                            for ni in 0..self.n {
                                // update board
                                self.board[ni][j] = if ni + cost.0 < self.n {
                                    self.board[ni + cost.0][j]
                                } else {
                                    None
                                };
                            }
                            // add output
                            for _ in 0..cost.0 {
                                output.push(OutputItem {
                                    d: Direction::Up,
                                    p: j,
                                });
                            }
                        }
                        Direction::Down => {
                            for ni in (0..self.n).rev() {
                                // update board
                                self.board[ni][j] = if ni >= cost.0 {
                                    self.board[ni - cost.0][j]
                                } else {
                                    None
                                };
                            }
                            // add output
                            for _ in 0..cost.0 {
                                output.push(OutputItem {
                                    d: Direction::Down,
                                    p: j,
                                });
                            }
                        }
                        Direction::Left => {
                            for nj in 0..self.n {
                                // update board
                                self.board[i][nj] = if nj + cost.0 < self.n {
                                    self.board[i][nj + cost.0]
                                } else {
                                    None
                                };
                            }
                            // add output
                            for _ in 0..cost.0 {
                                output.push(OutputItem {
                                    d: Direction::Left,
                                    p: i,
                                });
                            }
                        }
                        Direction::Right => {
                            for nj in (0..self.n).rev() {
                                // update board
                                self.board[i][nj] = if nj >= cost.0 {
                                    self.board[i][nj - cost.0]
                                } else {
                                    None
                                };
                            }
                            // add output
                            for _ in 0..cost.0 {
                                output.push(OutputItem {
                                    d: Direction::Right,
                                    p: i,
                                });
                            }
                        }
                    }
                    continue 'outer;
                }
            }
        }
        Output(output)
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
