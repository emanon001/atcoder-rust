#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn matched(i: usize, num: usize, grid: &[Vec<char>], numbers: &[Vec<char>]) -> bool {
    for r in 0..5 {
        for c in 0..4 {
            if grid[r][i * 4 + c] != numbers[r][num * 4 + c] {
                return false;
            }
        }
    }
    true
}

fn solve() {
    input! {
        n: usize,
        grid: [Chars; 5]
    };

    let numbers = r#"
.###..#..###.###.#.#.###.###.###.###.###.
.#.#.##....#...#.#.#.#...#.....#.#.#.#.#.
.#.#..#..###.###.###.###.###...#.###.###.
.#.#..#..#.....#...#...#.#.#...#.#.#...#.
.###.###.###.###...#.###.###...#.###.###.
    "#
    .trim()
    .split("\n")
    .map(|row| row.chars().collect::<Vec<char>>())
    .collect::<Vec<_>>();
    let res = (0..n)
        .map(|i| {
            for num in 0..10 {
                if matched(i, num, &grid, &numbers) {
                    return num;
                }
            }
            unreachable!();
        })
        .join("");
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
