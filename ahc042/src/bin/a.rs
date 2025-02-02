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
        let mut max_count = 4 * self.n.pow(2);
        let mut count = 0;
        let mut output = Vec::new();
        loop {
            if count == max_count {
                break;
            }

            count += 1;
        }
        Output(output)
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
