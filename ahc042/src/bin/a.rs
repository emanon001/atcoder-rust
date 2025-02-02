#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Piece {
    Oni,
    Fuku,
}

struct Solver {
    n: usize,
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

        Self { n, board }
    }
    fn solve(&mut self) {
        todo!();
    }
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        C: [Chars; N],
    };

    let mut solver = Solver::new(N, C);
    solver.solve();
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
