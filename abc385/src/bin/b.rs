#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: usize, W: usize, X: Usize1, Y: Usize1,
        S: [Chars; H],
        T: Chars,
    };

    let mut house_set = HashSet::new();
    let mut pos = (X, Y);
    for t in T {
        let x = pos.0 as isize;
        let y = pos.1 as isize;
        let (new_x, new_y) = match t {
            'U' => (x - 1, y),
            'D' => (x + 1, y),
            'L' => (x, y - 1),
            'R' => (x, y + 1),
            _ => unreachable!(),
        };
        if new_x < 0 || new_x >= H as isize || new_y < 0 || new_y >= W as isize {
            continue;
        }
        let x = new_x as usize;
        let y = new_y as usize;
        if S[x][y] == '#' {
            continue;
        }
        if S[x][y] == '@' {
            house_set.insert((x, y));
        }
        pos = (x, y);
    }
    println!("{} {} {}", pos.0 + 1, pos.1 + 1, house_set.len());
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
